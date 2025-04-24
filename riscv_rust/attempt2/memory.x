MEMORY
{
  ROM : ORIGIN = 0x80000000, LENGTH = 2M
  RAM : ORIGIN = 0x80200000, LENGTH = 2M
}

SECTIONS
{
  .text : {
    KEEP(*(.text.boot))
    *(.text*)
  } > ROM

  .rodata : { *(.rodata*) } > ROM
  .data : { *(.data*) } > RAM
  .bss : { *(.bss*) *(COMMON) } > RAM
}
