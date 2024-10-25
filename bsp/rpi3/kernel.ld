PAGE_SIZE = 64K;
PAGE_MASK = PAGE_SIZE - 1;

/* Physical address of the start of DRAM */
__rpi_phys_dram_start = 0;

/* Physical address that the binary will be loaded into memory */
__rpi_phys_binary_load_addr = 0x80000;

ENTRY(__rpi_phys_binary_load_addr)

/* Configure program headers */
PHDRS
{
    /* Set R/W permissions for the `segment_boot_core_stack` section */
    segment_boot_core_stack PT_LOAD FLAGS(6);

    /* Set R/X permissions for the `segment_code` section */
    segment_code PT_LOAD FLAGS(5);
}

SECTIONS
{
    /* Begin mapping memory at the start of physical RAM */
    . = __rpi_phys_dram_start;

    /* Place the boot stack at the start of RAM. Memory mapped peripherals may exist here, however
       the stack begins at the bottom and grows towards 0, so it's unlikely to exhaust the entire
       stack space. */
    .boot_core_stack (NOLOAD) :
    {
        // Capture the top of the stack
        __boot_core_stack_start = .;

        // Allocate all the space until the binary load address to the stack
        . += __rpi_phys_binary_load_addr;

        // Capture the bottom of the stack
        __boot_core_stack_end_exclusive = .;
    } :segment_boot_core_stack

    ASSERT((. & PAGE_MASK) == 0, "End of boot core stack is not page aligned")

    . = __rpi_phys_binary_load_addr;

    .text :
    {
        KEEP(*(.text._start))
        *(.text.__start_rust)
        *(.text*)
    } :segment_code

    .bss (NOLOAD) : ALIGN(16)
    {
        __bss_start = .;
        *(.bss*);
        . = ALIGN(16);
        __bss_end_exclusive = .;
    }
}
