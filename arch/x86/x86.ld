OUTPUT_FORMAT("elf32-i386", "elf32-i386", "elf32-i386")
OUTPUT_ARCH(i386)

ENTRY(_start)

_estack = ORIGIN(RAM) + LENGTH(RAM);

MEMORY
{
    DOS (rw)  : ORIGIN =       0, LENGTH = 1M
    VGA (rw)  : ORIGIN = 0xb8000, LENGTH = 80*25*2
    RAM (xrw) : ORIGIN =      1M, LENGTH = 1M
}

SECTIONS {
    /* . = 1M; */

    . = ALIGN(0x10);

    .text :
    {
        . = ALIGN(0x10);
        *(.*boot*)
        *(.text .text.* .gnu.linkonce.t.*)
    } >RAM

    .vga :
    {
        *(.vga*)
    } >VGA
}
