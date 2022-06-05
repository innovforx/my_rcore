mod context;
use crate::{Errorln, loader::*, syscall, timer::set_next_trigger, 
    task::{suspend_current_and_run_next,current_app_token},
     println, config::{TRAMPLINE, TRAP_CONTEXT}, Debugln};
pub use crate::trap::context::TrapContext;

use core::{arch::{global_asm, asm}};
use crate::syscall::syscall;
use riscv::register::{
    self,
    mtvec::TrapMode,
    scause::{
        self,
        Exception,
        Trap,
        Interrupt,
    },
    stval,
    stvec, sie,
};

global_asm!(include_str!("trap.S"));

pub fn init(){
    set_kernel_trap_entry();
}

fn set_kernel_trap_entry(){
    unsafe{
        stvec::write( trap_from_kernel as usize ,TrapMode::Direct);
    }
}

fn trap_from_kernel() -> !{
    panic!("trap from kernel not impl");
}

fn set_user_trap_entry(){
    unsafe{
        stvec::write(TRAMPLINE,TrapMode::Direct);
    }
}

pub fn enable_time_handler(){
    unsafe{
        sie::set_stimer();
    }
}

#[no_mangle]
pub fn trap_handler(cx : &mut TrapContext) -> !{
    set_kernel_trap_entry();
    let scause = scause::read();
    let stval = stval::read();

    match scause.cause(){
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            suspend_current_and_run_next();
        },

        Trap::Exception(Exception::UserEnvCall) => {
            //下一条指令的地址
            cx.sepc += 4;
            //通过系统调用做转换
            cx.x[10] = syscall(cx.x[17],[cx.x[10],cx.x[11],cx.x[12]]) as usize;

        },
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) | 
        Trap::Exception(Exception::LoadFault) | Trap::Exception(Exception::LoadPageFault)=>{
            Errorln!("[kernel] PageFault in application, kernel killed it.");
            crate::task::exit_current_and_run_next();
            // run_next_app();
        },
        Trap::Exception(Exception::IllegalInstruction) => {
            Errorln!("[kernel] IllegalInstruction in application, kernel killed it.");
            crate::task::exit_current_and_run_next();
            // run_next_app();
        },
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    trap_return();
}

#[no_mangle]
pub fn trap_return() -> !{
    Debugln!("in trap return");
    set_user_trap_entry();
    let trap_cx_ptr = TRAP_CONTEXT;
    let user_satp = current_app_token();
    extern "C"{
        fn __alltraps();
        fn __restore();
    }

    let restore_va = __restore as usize - __alltraps as usize + TRAMPLINE;
    Debugln!("0X{:x}",restore_va);
    // unsafe{
    //     asm!{
    //         "fence.i",
    //         "jr {restore_va}",
    //         restore_va = in(reg) restore_va,
    //         in("a0") trap_cx_ptr,
    //         in("a1") user_satp,
    //         options(noreturn)
    //     };
    // };
    
    unsafe {
        asm!(
            "fence.i",
            "jr {restore_va}",             // jump to new addr of __restore asm function
            restore_va = in(reg) restore_va,
            in("a0") trap_cx_ptr,      // a0 = virt addr of Trap Context
            in("a1") user_satp,        // a1 = phy addr of usr page table
            options(noreturn)
        );
    }
}