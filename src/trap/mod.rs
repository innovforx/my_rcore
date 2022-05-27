mod context;
use crate::{Errorln, loader::*, syscall, timer::set_next_trigger, task::suspend_current_and_run_next, println};
pub use crate::trap::context::TrapContext;

use core::arch::global_asm;
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
    extern "C" {
        fn __alltraps();
    }
    unsafe{
        stvec::write(__alltraps as usize, TrapMode::Direct );
    }
}

pub fn enable_time_handler(){
    unsafe{
        sie::set_stimer();
    }
}

#[no_mangle]
pub fn trap_handler(cx : &mut TrapContext) -> &mut TrapContext{
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
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault)=>{
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
    cx
}