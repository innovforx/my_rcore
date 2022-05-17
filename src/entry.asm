    .section .text.entry
    .global _start
_start:
    # li x1, 100 # x1 =100 
    la sp,boot_stack_top
    call rust_main
    # li x1,100  # li load immediate

    .section .bss.stack
    .global boot_stack
boot_stack:
    .space 4096*16
    .global boot_stack_top
boot_stack_top:
