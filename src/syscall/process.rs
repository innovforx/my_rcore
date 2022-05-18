//! App management syscalls
// use crate::batch::run_next_app;

use crate::println;
use crate::batch::run_next_app;
use crate::batch::print_current_app_info;
/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    run_next_app()
}

pub fn sys_app_info(id : usize) -> isize{
    
    print_current_app_info();
    
    0
}
