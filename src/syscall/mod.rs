

mod fs;
mod process;

use fs::*;
use process::*;

use crate::{Errorln, println};



const SYSCALL_WRITE:usize = 2;
const SYSCALL_EXIT:usize = 93;
const SYSCALL_YIELD:usize = 124;
pub fn syscall(syscall_id : usize,args : [usize;3]) -> isize{
    match syscall_id {
        SYSCALL_WRITE => {
            // println!("fd : {}",args[0]);
            sys_write(args[0],args[1] as *const u8, args[2])
        },
        SYSCALL_EXIT => {
            sys_exit(args[0] as i32)
        },

        SYSCALL_YIELD =>{
            sys_yield()
        },

        _ => {
            Errorln!("not support syscall id");
            panic!("not support syscall id")
        },
    }
    
}