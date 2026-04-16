ENTRY(kernel_main)

SECTIONS {
    . = 0x100000;

    .text   : { *(.text .text.*) }
    .rodata : { *(.rodata .rodata.*) }
    .data   : { *(.data .data.*) }
    .bss    : {
        __bss_start = .;
        *(.bss .bss.* COMMON)
        __bss_end = .;
    }

    /DISCARD/ : { *(.eh_frame) *(.note*) }
}
