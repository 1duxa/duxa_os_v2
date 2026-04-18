ENTRY(kernel_main)

SECTIONS {
    . = 0x100000;
    _kernel_start = .;
    .text   : { *(.text .text.*) }
    .rodata : { *(.rodata .rodata.*) }
    .data   : { *(.data .data.*) }
    .bss    : {
        __bss_start = .;
        *(.bss .bss.* COMMON)
        __bss_end = .;
    }
    _kernel_end = .;
    /DISCARD/ : { *(.eh_frame) *(.note*) }
}
