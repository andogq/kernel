/* Physical address that the binary will be loaded into memory */
__rpi_phys_binary_load_addr = 0x80000;

ENTRY(__rpi_phys_binary_load_addr)

/* Configure program headers */
PHDRS
{
    /* Set R/X permissions for the `segment_code` section */
    segment_code PT_LOAD FLAGS(5);
}


SECTIONS
{
    . = __rpi_phys_binary_load_addr;

    .text :
    {
        KEEP(*(.text._start))
        *(.text*)
    } :segment_code
}
