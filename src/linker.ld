OUTPUT_ARCH(riscv)
ENTRY(_start)

BASE_ADDRESS = 0x80200000;

SECTIONS {

  . = BASE_ADDRESS;

  stext = .;
  .text : {
    *(.text.entry)
    *(.text, .text.*)
  }
  etext = .;

  srodata = .;
  .rodata : {
    *(.rodata, .rodata.*)
  }
  erodata = .;

  sdata = .;
  .data : {
    *(.data, .data.*)
  }
  edata = .;

  sbss = .;
  .bss : {
    *(.bss.stack)
    *(.bss, .bss.*)
  }
  ebss = .;

}
