//! This [`entry`] acts as the very first entry point for the Aarch64 architecture. At this time,
//! the CPU will be in EL2, the MMU will be disabled, and all CPU cores will be attempting to boot.
//!
//! This entry point will:
//!
//! - Halt any non-boot cores
//!
//! - Configure the MMU to map the kernel (and stack, etc) into the upper half of memory
//!
//! - Jump to `__start_rust`
//!
//! # Symbols
//!
//! The entry point requires the following symbols to be available:
//!
//! - `__kernel_start`: The start (low) physical address of the kernel memory
//! - `__kernel_end`: The end (high) physical address of the kernel memory
//! - `__kernel_stack_start`: The start physical address of the kernel memory
//! - `__kernel_stack_end`: The end physical address of the kernel memory
//! - `__start_rust`: Location to jump to once MMU is activated

#![no_std]

use core::cell::UnsafeCell;

use aarch64_cpu::registers::*;
use bitfield::bitfield;

extern "C" {
    /// Physical start address of the kernel.
    static __kernel_start: UnsafeCell<()>;
    /// Physical end address of the kernel.
    static __kernel_end: UnsafeCell<()>;
    /// Physical start address of the kernel stack.
    static __kernel_stack_start: UnsafeCell<()>;
    /// Physical end address of the kernel stack.
    static __kernel_stack_end: UnsafeCell<()>;
    /// Symbol to jump to after initialisation.
    static __start_rust: UnsafeCell<()>;
}

/// 64kB page size.
const PAGE_SIZE: u64 = 65_536; // TODO: Migrate to _anywhere_ else
const PAGE_SHIFT: usize = PAGE_SIZE.ilog2() as usize;

#[repr(transparent)]
struct L1TranslationTable([TableDescriptor; 1024]);
impl L1TranslationTable {
    pub const fn new() -> Self {
        Self([const { TableDescriptor(0) }; 1024])
    }
}
#[repr(transparent)]
struct L2TranslationTable([TableDescriptor; 8192]);
impl L2TranslationTable {
    pub const fn new() -> Self {
        Self([const { TableDescriptor(0) }; 8192])
    }
}
#[repr(transparent)]
struct L3TranslationTable([PageDescriptor; 8192]);
impl L3TranslationTable {
    pub const fn new() -> Self {
        Self([const { PageDescriptor(0) }; 8192])
    }
}

const L2_TABLE_COUNT: usize = 5;
const L3_TABLE_COUNT: usize = 5;

// TODO: Interior mutability
// TODO: Point `TTBR_ELx` to this
static mut L1_TRANSLATION_TABLE: L1TranslationTable = L1TranslationTable::new();
static mut L2_TRANSLATION_TABLES: [L2TranslationTable; L2_TABLE_COUNT] =
    [const { L2TranslationTable::new() }; L2_TABLE_COUNT];
static mut L3_TRANSLATION_TABLES: [L3TranslationTable; L3_TABLE_COUNT] =
    [const { L3TranslationTable::new() }; L3_TABLE_COUNT];

/// Contains all information required to map a physical portion of memory into a virtual address
/// space.
struct MemoryMapDescriptor {
    /// The physical address to map to.
    physical_address: u64,
    /// The virtual address to map from.
    virtual_address: u64,
    /// Size of the descriptor.
    size: u64,
}

impl MemoryMapDescriptor {
    /// Generate a descriptor from a set of symbols and a target virtual address.
    pub fn from_symbols(
        virtual_address: u64,
        start: &UnsafeCell<()>,
        end: &UnsafeCell<()>,
    ) -> Self {
        let start = start.get() as u64;
        let end = end.get() as u64;

        Self {
            physical_address: start,
            virtual_address,
            size: end - start,
        }
    }

    /// Produce an iterator that steps through each page, with the virtual address and the
    /// corresponding physical address for the start of each page.
    pub fn pages(&self) -> impl Iterator<Item = (TranslationAddress, u64)> {
        (self.virtual_address..)
            .zip(self.physical_address..)
            .take(self.size as usize)
            .step_by(PAGE_SIZE as usize)
            .map(|(virt, phys)| (TranslationAddress(virt), phys))
    }
}

bitfield! {
    pub struct TableDescriptor(u64);
    pub ns_table, set_ns_table: 63;
    pub ap_table, set_ap_table: 62, 61;
    pub xn_table, set_xn_table: 60;
    pub pxn_table, set_pxn_table: 59;
    pub u32, mask ADDRESS_MASK(u64), next_address, set_next_address: 47, 16;
    pub marker, set_marker: 1, 0;
}

impl TableDescriptor {
    /// Bits required to indicate this is a valid table descriptor.
    const VALID_BITS: u64 = 0b11;

    /// Determine if this table descriptor is a valid descriptor.
    pub fn valid(&self) -> bool {
        // Figure D8-12: Bits [1:0] are `11` if the table descriptor is valid.
        self.marker() == Self::VALID_BITS
    }

    /// Configure descriptor bits to make this a valid table descriptor.
    pub fn set_valid(&mut self) {
        self.set_marker(Self::VALID_BITS);
    }
}

bitfield! {
    pub struct PageDescriptor(u64);

    /// Bits [63:32] of the physical address that contains this page.
    pub u32, mask ADDRESS_MASK(u64), output_address, set_output_address: 47, 16;

    /// Indicates one of the following:
    ///
    /// - `0`: The memory region has not been accessed since the value of AF was last set to `0`.
    ///
    /// - `1`: The memory region has been accessed since the value of AF was last set to `0`.
    ///
    /// Descriptors with AF set to `0` can never be cached in a TLB.
    ///
    /// _from ARM ARM D8.4.5_
    pub access_flag, set_access_flag: 10;

    pub marker, set_marker: 1, 0;
}

impl PageDescriptor {
    /// Bits required to indicate this is a valid page descriptor.
    const VALID_BITS: u64 = 0b01;

    /// Determine whether this page descriptor is valid.
    pub fn valid(&self) -> bool {
        self.marker() == Self::VALID_BITS
    }

    /// Configure descriptor bits to make this a valid page descriptor.
    pub fn set_valid(&mut self) {
        self.set_marker(Self::VALID_BITS);
    }
}

bitfield! {
    /// Converts an address into the respective indexes for each translation table granule.
    ///
    /// Note: These mappings are only suitable for 64kB granule size with 48 bit input addresses.
    struct TranslationAddress(u64);

    /// Original virtual address.
    address, _: 63, 0;

    // TODO: Do some new-type stuff on these indexes (eg `Index<L1>(u64)`)

    /// Index to lookup in the level 1 translation table.
    l1_index, _: 47, 42;
    /// Index to lookup in the level 2 translation table.
    l2_index, _: 41, 29;
    /// Index to lookup in the level 3 translation table.
    l3_index, _: 28, 16;
}

/// Manages ownership and delegates mutable access to a collection of 'slots' backed by some static
/// slice. This can be used to track which slots are available, and claim them so that they can
/// only be used by a single user. The memory address of the slots is also tracked, and can be used
/// to retrieve a slot for a specific address.
struct AddressedSlots<const SLOTS: usize, T: 'static> {
    /// Mutable reference to a backing slice.
    backing: &'static mut [T; SLOTS],

    /// Contains the address of the slot for the corresponding index if it is not used.
    slots: [Option<u32>; SLOTS],
}

impl<const SLOTS: usize, T: 'static> AddressedSlots<SLOTS, T> {
    /// Create a new instance with the provided mutable slice.
    pub const fn new(backing: &'static mut [T; SLOTS]) -> Self {
        Self {
            backing,
            slots: [None; SLOTS],
        }
    }

    /// Request a new slot. If one is availble, it's address in addition to a mutable reference
    /// will be provided.
    pub fn new_slot(&mut self) -> Option<(u32, &mut T)> {
        // Find the next available slot
        let (index, slot) = self
            .slots
            .iter_mut()
            .enumerate()
            .find(|(_, slot)| slot.is_none())
            .expect("not enough slots provided to create new slot");

        let item = &mut self.backing[index];

        // Determine it's address
        let addr = item as *const _ as u32;

        // Save the address in the lookup table
        *slot = Some(addr);

        Some((addr, item))
    }

    /// Attempt to retrieve a mutable reference to a slot corresponding to a specific address.
    ///
    /// This is intended to be a safe alternative to casting and dereferencing a random address.
    pub fn fetch_for_address(&mut self, address: u32) -> Option<&mut T> {
        self.slots
            .iter()
            .enumerate()
            .flat_map(|(i, addr)| Some((i, addr.as_ref()?)))
            .find(|(_, addr)| **addr == address)
            .map(|(i, _)| &mut self.backing[i])
    }
}

/// Entry point for Aarch64.
#[inline(always)]
pub extern "C" fn entry() -> ! {
    // TODO: Still need very-early code to set up stack pointer

    // Safety: The linker is responsible for inserting correct addresses for the requested symbols.
    // Invalid addresses will lead to random parts of memory being mapped.
    let descriptors = unsafe {
        [
            MemoryMapDescriptor::from_symbols(0xdeadbeef, &__kernel_start, &__kernel_stack_end),
            MemoryMapDescriptor::from_symbols(
                0xffffffff,
                &__kernel_stack_start,
                &__kernel_stack_end,
            ),
        ]
    };

    // WARN: This could be _much_ smarter

    // Safety: Only basic atomic operations (reading/writing u64) will take place, and nothing else
    // will be accessing this memory until it's pointer is loaded into the CPU.
    #[allow(static_mut_refs)]
    let l1_table = unsafe { &mut L1_TRANSLATION_TABLE.0 };

    let mut l2_slots = AddressedSlots::new(
        // Safety: Mutable reference to a mutable static is required as it must be built in place
        // as absolute addresses must be calculated. Interacting through `AddressedSlots` limits
        // the surface area for mistakes, and nothing else should be using this memory until the
        // address to it is loaded into a specific register.
        #[allow(static_mut_refs)]
        unsafe {
            &mut L2_TRANSLATION_TABLES
        },
    );

    let mut l3_slots = AddressedSlots::new(
        // Safety: See above.
        #[allow(static_mut_refs)]
        unsafe {
            &mut L3_TRANSLATION_TABLES
        },
    );

    for descriptor in descriptors {
        for (virt, phys) in descriptor.pages() {
            // Shifted address which is stored in the page descriptor
            let phys_upper = (phys >> PAGE_SHIFT) as u32;

            let l1_descriptor = &mut l1_table[virt.l1_index() as usize];

            let l2_table = if !l1_descriptor.valid() {
                // Fetch a new l2 table
                let (addr, l2_table) = l2_slots
                    .new_slot()
                    .expect("pre-allocated l2 tables to be enough");

                // Shift address to get the 'page id'
                let addr = addr >> PAGE_SHIFT;

                // Save the address of the l2 table into the descriptor
                l1_descriptor.set_next_address(addr);

                // TODO: Other descriptor setup here (flags?)
                l1_descriptor.set_valid();

                l2_table
            } else {
                let addr = l1_descriptor.next_address();

                // Shift address to get the 'page id'
                let addr = addr >> PAGE_SHIFT;

                l2_slots
                    .fetch_for_address(addr)
                    .expect("next address stored in l1 descriptor must be valid")
            };

            let l2_descriptor = &mut l2_table.0[virt.l2_index() as usize];

            let l3_table = if !l2_descriptor.valid() {
                // Fetch a new l3 table
                let (addr, l3_table) = l3_slots
                    .new_slot()
                    .expect("pre-allocated l3 tables to be enough");

                // Shift address to get the 'page id'
                let addr = addr >> PAGE_SHIFT;

                // Save the address of the l3 table into the descriptor
                l2_descriptor.set_next_address(addr);

                // TODO: Other descriptor setup here (flags?)
                l2_descriptor.set_valid();

                l3_table
            } else {
                let addr = l2_descriptor.next_address();

                // Shift address to get the 'page id'
                let addr = addr >> PAGE_SHIFT;

                l3_slots
                    .fetch_for_address(addr)
                    .expect("next address stored in l2 descriptor must be valid")
            };

            let l3_descriptor = &mut l3_table.0[virt.l3_index() as usize];

            if l3_descriptor.valid() {
                if l3_descriptor.output_address() != phys_upper {
                    // Attempting to re-map existing physical address to different physical address
                    panic!("page table clash");
                }

                // Page already mapped
                continue;
            }

            // Set descriptor flags
            l3_descriptor.set_valid();
            l3_descriptor.set_access_flag(true);
        }
    }

    // Activate the MMU

    // TODO: Setup MAIR register
    MAIR_EL1.write(
        MAIR_EL1::Attr0_Device::nonGathering_nonReordering_noEarlyWriteAck
            + MAIR_EL1::Attr1_Normal_Outer::NonCacheable
            + MAIR_EL1::Attr1_Normal_Inner::NonCacheable
            + MAIR_EL1::Attr2_Normal_Outer::WriteBack_NonTransient
            + MAIR_EL1::Attr2_Normal_Inner::WriteBack_NonTransient,
    );

    // Jump to somewhere
    loop {}
}
