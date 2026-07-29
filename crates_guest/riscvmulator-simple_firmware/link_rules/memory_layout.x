ENTRY(_start)
MEMORY {
  RAM (rwx) : ORIGIN = 0x80000000, LENGTH = 64K
}
SECTIONS {
  .text   : { *(.text.init) *(.text .text.*) } > RAM
  .rodata : { *(.rodata .rodata.*) }           > RAM
  .data   : { *(.data .data.*) }               > RAM
  .bss    : { *(.bss .bss.*) *(COMMON) }       > RAM
  . = ALIGN(16);
  __stack_top = ORIGIN(RAM) + LENGTH(RAM);
}
