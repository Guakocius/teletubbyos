OUTPUT_FORMAT(elf64-x86-64)
ENTRY(_start)

PHDRS
{
    kernel PT_LOAD FLAGS(5); /* R + X */
    rodata PT_LOAD FLAGS(4); /* R */
    data   PT_LOAD FLAGS(6); /* R + W */
}

KERNEL_VMA = 0xffffffff80000000;
KERNEL_LMA = 0x100000;

SECTIONS
{
    /* physical load base */
    . = KERNEL_LMA;

    /* ---------- TEXT ---------- */
    .text : ALIGN(0x1000)
    {
        *(.text .text.*)
    } :kernel

    /* ---------- RODATA ---------- */
    .rodata : ALIGN(0x1000)
    {
        KEEP(*(.limine_requests))
        *(.rodata .rodata.*)
    } :rodata

    /* ---------- DATA ---------- */
    .data : ALIGN(0x1000)
    {
        *(.data .data.*)
    } :data

    /* ---------- BSS ---------- */
    .bss : ALIGN(0x1000)
    {
        *(COMMON)
        *(.bss .bss.*)
    } :data

    /DISCARD/ :
    {
        *(.eh_frame .eh_frame_hdr .comment .note .note.*)
    }
}
