#![no_std]

use core::{fmt, marker::PhantomData};

use tock_registers::{interfaces::*, register_bitfields, register_structs, registers::*};

pub enum Uninitialised {}
pub enum Initialised {}

pub struct Pl011<const BASE_ADDRESS: usize, I = Uninitialised> {
    _init_state: PhantomData<I>,
}

impl<const BASE_ADDRESS: usize, I> Pl011<BASE_ADDRESS, I> {
    /// Create a new, uninitialised PL011 instance.
    pub fn new() -> Pl011<BASE_ADDRESS, Uninitialised> {
        Pl011 {
            _init_state: PhantomData,
        }
    }

    /// Fetch the register block of this instance.
    ///
    /// # Safety:
    ///
    /// `BASE_ADDRESS` must be a valid memory address, and point to the start of the memory-mapped
    /// registers for this instance of the PL011 peripheral.
    unsafe fn registers(&self) -> &'static mut RegisterBlock {
        &mut *(BASE_ADDRESS as *mut RegisterBlock)
    }
}

impl<const BASE_ADDRESS: usize> Pl011<BASE_ADDRESS, Uninitialised> {
    /// Initialise the peripheral instance
    pub fn initialise(self) -> Pl011<BASE_ADDRESS, Initialised> {
        let registers = unsafe { self.registers() };

        // Disable the peripheral incase it's already on
        registers.CR.set(0);

        // Ensure interrupts are cleared
        registers.ICR.write(ICR::ALL::CLEAR);

        // Configure UART For 921_600 baud and 8N1
        registers.IRBD.write(IBRD::BAUD_DIVINT.val(3));
        registers.FRBD.write(FBRD::BAUD_DIVFRAC.val(16));

        // Configure FIFO
        registers
            .LCR_H
            .write(LCR_H::WLEN::EightBit + LCR_H::FEN::FifosEnabled);
        registers.IFLS.write(IFLS::RXIFLSEL::OneEighth);

        // Enable interrupts for receive and receive timeout
        registers
            .IMSC
            .write(IMSC::RXIM::Enabled + IMSC::RTIM::Enabled);

        // Turn on the UART
        registers
            .CR
            .write(CR::UARTEN::Enabled + CR::TXE::Enabled + CR::RXE::Enabled);

        Pl011 {
            _init_state: PhantomData,
        }
    }
}

impl<const BASE_ADDRESS: usize> Pl011<BASE_ADDRESS, Initialised> {
    /// Block until all bytes have been transmitted.
    fn flush(&self) {
        let registers = unsafe { self.registers() };

        while registers.FR.matches_all(FR::TXFF::SET) {
            // TODO: nop
        }
    }

    fn write_char(&mut self, c: char) {
        let registers = unsafe { self.registers() };

        // TODO: Wait for TX FIFO to havea slot

        registers.DR.set(c as u32);
    }
}

impl<const BASE_ADDRESS: usize> fmt::Write for Pl011<BASE_ADDRESS, Initialised> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }

        Ok(())
    }
}

register_bitfields! {
    u32,

    /// Flag Register
    FR [
        /// Transmit FIFO empty
        TXFE OFFSET(7) NUMBITS(1) [],
        /// Transmit FIFO full
        TXFF OFFSET(5) NUMBITS(1) [],
        /// Receive FIFO empty
        RXFE OFFSET(4) NUMBITS(1) [],
        /// UART busy
        BUSY OFFSET(3) NUMBITS(1) [],
    ],

    /// Integer Baud Rate Divisor
    IBRD [
        /// The integer baud rate divisor
        BAUD_DIVINT OFFSET(0) NUMBITS(16) [],
    ],

    /// Fractional Baud Rate Divisor
    FBRD [
        BAUD_DIVFRAC OFFSET(0) NUMBITS(6) [],
    ],

    /// Line Control Register
    LCR_H [
        /// Word length
        WLEN OFFSET(5) NUMBITS(2) [
            FiveBit = 0b00,
            SixBit = 0b01,
            SevenBit = 0b10,
            EightBit = 0b11,
        ],

        /// Enable FIFOs
        FEN OFFSET(4) NUMBITS(1) [
            FifosDisabled = 0,
            FifosEnabled = 1,
        ],
    ],

    /// Control Register
    CR [
        /// Receive enable
        RXE OFFSET(9) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],

        /// Transmit enable
        TXE OFFSET(8) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],

        /// UART enable
        UARTEN OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],
    ],

    /// Interrupt FIFO Level Select Register
    IFLS [
        /// Receive interrupt FIFO level select. The trigger points for the receive interrupt are
        /// as follows.
        RXIFLSEL OFFSET(3) NUMBITS(5) [
            OneEighth = 0b000,
            OneQuarter = 0b001,
            OneHalf = 0b010,
            ThreeQuarters = 0b011,
            SevenEighths = 0b100,
        ],
    ],

    /// Interrupt Mask Set/Clear Register
    IMSC [
        /// Receive timeout interrupt mask. A read returns the current mask for the UARTRTINTR
        /// interrupt.
        ///
        /// - On a write of 1, the mask of the UARTRTINTR interrupt is set
        /// - A write of 0 clears the mask.
        RTIM OFFSET(6) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],

        /// Receive interrupt mask. A read returns the current mask for the UARTRXINTR interrupt.
        ///
        /// - On a write of 1, the mask of the UARTRXINTR interrupt is set.
        /// - A write of 0 clears the mask.
        RXIM OFFSET(4) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],
    ],

    /// Mask Interrupt Status Register
    MIS [
        /// Receive timeout masked interrupt status. Returns the masked interrupt state of the
        /// UARTRTINTR interrupt.
        RTMIS OFFSET(6) NUMBITS(1) [],

        /// Receive masked interrupt status. Returns the masked interrupt state of the UARTRXINTR
        /// interrupt.
        RXMIS OFFSET(4) NUMBITS(1) [],
    ],

    /// Interrupt Clear Register
    ICR [
        /// Meta field for all pending interrupts.
        ALL OFFSET(0) NUMBITS(11) [],
    ],
}

register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x00 => DR: ReadWrite<u32>),
        (0x04 => _reserved1),
        (0x18 => FR: ReadOnly<u32, FR::Register>),
        (0x1C => _reserved2),
        (0x24 => IRBD: WriteOnly<u32, IBRD::Register>),
        (0x28 => FRBD: WriteOnly<u32, FBRD::Register>),
        (0x2C => LCR_H: WriteOnly<u32, LCR_H::Register>),
        (0x30 => CR: ReadWrite<u32, CR::Register>),
        (0x34 => IFLS: ReadWrite<u32, IFLS::Register>),
        (0x38 => IMSC: ReadWrite<u32, IMSC::Register>),
        (0x3C => _reserved3),
        (0x40 => MIS: ReadOnly<u32, MIS::Register>),
        (0x44 => ICR: WriteOnly<u32, ICR::Register>),
        (0x48 => @END),
    }
}
