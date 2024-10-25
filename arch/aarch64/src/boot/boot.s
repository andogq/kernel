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

2:
    nop
    wfe
    nop
    b 2b

9:                  // <- Loop
    wfe
    b 9b            // ^  Loop
