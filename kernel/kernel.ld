ENTRY(kernel_main)
PHYS_BASE = 0x100000;
VIRT_BASE = 0xFFFFFFFF80100000;

SECTIONS {
    . = VIRT_BASE;
    _kernel_start = .;

    .text    : AT(PHYS_BASE + (. - VIRT_BASE)) { *(.text .text.*)       }
    .rodata  : AT(PHYS_BASE + (. - VIRT_BASE)) { *(.rodata .rodata.*)   }
    .data    : AT(PHYS_BASE + (. - VIRT_BASE)) { *(.data .data.*)       }
    .got     : AT(PHYS_BASE + (. - VIRT_BASE)) { *(.got .got.*)         }
    .got.plt : AT(PHYS_BASE + (. - VIRT_BASE)) { *(.got.plt)            }

    .bss : {
        __bss_start = .;
        *(.bss .bss.* COMMON)
        __bss_end = .;
    }

    _kernel_end = .;
    /DISCARD/ : { *(.eh_frame) *(.note*) *(.comment) *(.gnu.hash) *(.dynsym) *(.dynstr) *(.dynamic) *(.rela.*) }
}
