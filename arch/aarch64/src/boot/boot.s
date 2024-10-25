// Load the relative address (to the PC) of a value.
// The symbol must be within +/- 4Gb of the PC.
.macro ADR_REL register, symbol
    // An offset to the page into the upper part of the register
    adrp    \register, \symbol
    // Load the lower bits of the symbol address, which are the offset into the page.
    add     \register, \register, #:lo12:\symbol
.endm

1:
    // Core check: only boot if in EL2
    mrs     x0,     CurrentEL
    cmp     x0,     {CONST_CURRENTEL_EL2}
    b.ne    9f      // v  Loop

    // Core check: park if on non-boot core
    mrs     x1,     MPIDR_EL1
    and     x1, x1, {CONST_CORE_ID_MASK}
    cmp     x1,     {CONST_BOOT_CORE_ID}
    b.ne    9f      // v  Loop

    // Initialise BSS
    ADR_REL x0,     __bss_start
    ADR_REL x1,     __bss_end_exclusive

// Constantly loop to clear out BSS memory
1:
    // If the pointers meet each other, then BSS has been cleared.
    cmp     x0,     x1
    b.eq   2f

    // Zero out the current pointer location, and increment the pointer by 16 to the next position.
    stp     xzr, xzr, [x0], #16
    b 1b

2: // BSS complete, prepare for Rust
    // Set up the stack pointer
    ADR_REL x0, __boot_core_stack_end_exclusive
    mov     sp, x0

    b       _start_rust

9:                  // <- Loop
    wfe
    b 9b            // ^  Loop
