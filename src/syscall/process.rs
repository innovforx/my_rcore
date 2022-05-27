//! App management syscalls
// use crate::batch::run_next_app;



use crate::println;
// use crate::batch::run_next_app;
use crate::task::*;
use crate::timer::get_time_ms;
/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    // run_next_app()
    exit_current_and_run_next();
    panic!("unreachable code");    
}

pub fn sys_yield() -> isize{
    // println!("enter yield");
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time_ms() -> isize{
    get_time_ms() as isize
}

pub fn sys_get_info() -> isize{
    println!("{}",get_current_task_info());
    0
}