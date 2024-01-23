.section .text.entry, "ax"
.globl _start
_start:
  la sp, boot_stack_top
  call kernel_main
spin:
  j spin

.section .bss.stack
.globl boot_stack_lower_bound
boot_stack_lower_bound:
.space 4096 * 16
.globl boot_stack_top
boot_stack_top:
