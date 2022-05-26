.altmacro
.macro SAVE_SN n
    sd s\n, (\n+2)*8(a0)
.endm
.macro LOAD_SN n
    ld s\n, (\n+2)*8(a1)
.endm
    .section .text
    .globl switch

    # ; pub struct TaskContext {{}}
    # ; /// return address ( e.g. __restore ) of __switch ASM function
    # ; ra: usize,
    # ; /// kernel stack pointer of app
    # ; sp: usize,
    # ; /// callee saved registers:  s 0..11
    # ; s: [usize; 12],
    # ;
 
switch:
    # __switch(
    #     current_task_cx_ptr: *mut TaskContext,
    #     next_task_cx_ptr: *const TaskContext
    # )
    # save kernel stack of current task
    # store sp to mem(a0 + 8) 那么问题来了，即便保存了地址，又如何保证这个地址不会被覆盖 ，原因是这里面各个app的地址并不重合，依然是固定地址，所以不会被覆盖
    sd sp, 8(a0)
    # save ra & s0~s11 of current execution
    # s0 - s11 是保存寄存器
    # 第二个问题，为何只保存s0-s11这几个寄存器，ch2中任务切换的时候，保存了x0-x31
    # s0~s11 这些被调用者保存的寄存器。
    # 但 __switch 是一个用汇编代码写的特殊函数，它不会被 Rust/C 编译器处理，所以我们需要在 __switch 中手动编写保存 s0~s11 的汇编代码。 
    # 不用保存其它寄存器是因为：其它寄存器中，属于调用者保存的寄存器是由编译器在高级语言编写的调用函数中自动生成的代码来完成保存的；
    # 还有一些寄存器属于临时寄存器，不需要保存和恢复。
    sd ra, 0(a0)
    .set n, 0
    .rept 12
        SAVE_SN %n
        .set n, n + 1
    .endr
    # restore ra & s0~s11 of next execution
    ld ra, 0(a1)
    .set n, 0
    .rept 12
        LOAD_SN %n
        .set n, n + 1
    .endr
    # restore kernel stack of next task
    ld sp, 8(a1)
    ret

