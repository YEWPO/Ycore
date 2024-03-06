.altmacro
.macro SAVE_GRP n
  sd x\n, \n * 8(sp)
.endm
.macro LOAD_GRP n
  ld x\n, \n * 8(sp)
.endm

.section .text.trampoline
.align 3
.globl __alltraps
__alltraps:
  csrrw sp, sscratch, sp

  SAVE_GRP 1
  SAVE_GRP 3
  .set n, 5
  .rept 27
    SAVE_GRP %n
    .set n, n + 1
  .endr

  csrr t0, sstatus
  csrr t1, sepc
  csrr t2, sscratch
  sd t0, 32 * 8(sp)
  sd t1, 33 * 8(sp)
  sd t2, 2 * 8(sp)

  ld t0, 34 * 8(sp)
  ld t1, 35 * 8(sp)
  ld sp, 36 * 8(sp)

  csrw satp, t0
  sfence.vma
  jr t1

.globl __restore
__restore:
  csrw satp, a1
  sfence.vma
  csrw sscratch, a0
  mv sp, a0

  ld t0, 32 * 8(sp)
  ld t1, 33 * 8(sp)
  csrw sstatus, t0
  csrw sepc, t1

  LOAD_GRP 1
  LOAD_GRP 3
  .set n, 5
  .rept
    LOAD_GRP %n
    .set n, n + 1
  .endr
  ld sp, 2 * 8(sp)
  sret
