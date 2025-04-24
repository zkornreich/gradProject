OUTPUT_ARCH(riscv)
ENTRY(_start)

SECTIONS {
    . = 0x80000000; /* Default boot address for QEMU virt */
    .data : {
        *(.data*)
    }
    .text : {
        *(.text*)
    }
    .rodata : {
        *(.rodata*)
    }
    .bss : {
        *(.bss*)
        *(COMMON)
    }
}
