

mod fs;
mod process;

use fs::*;
use process::*;

use crate::{Errorln, println};
use crate::task::update_current_app_syscall_info;


const SYSCALL_WRITE:usize = 2;
const SYSCALL_EXIT:usize = 93;
const SYSCALL_YIELD:usize = 124;
const SYSCALL_GETTIME:usize = 169;
const SYSCALL_GETINFO : usize = 254;


pub const SYSCALL_NUM : usize = 4;



pub fn syscall(syscall_id : usize,args : [usize;3]) -> isize{
    update_current_app_syscall_info(syscall_id);
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
        SYSCALL_GETTIME => {
            sys_get_time_ms()
        },
        SYSCALL_GETINFO => {
            sys_get_info()
        }
        _ => {
            Errorln!("not support syscall id:{}",syscall_id);
            panic!("not support syscall id:{}",syscall_id)
        },
    }
    
}

pub fn get_syscall_name(syscall_id : usize) -> &'static str{
    match syscall_id {
        SYSCALL_WRITE => {
            // println!("fd : {}",args[0]);
            "SYSCALL_WRITE"
        },
        SYSCALL_EXIT => {
            "SYSCALL_EXIT"
        },

        SYSCALL_YIELD =>{
            "SYSCALL_YIELD"
        },
        SYSCALL_GETTIME => {
            "SYSCALL_GETTIME"
        },
        SYSCALL_GETINFO =>{
            "SYSCALL_GETINFO"
        }
        _ => {
            Errorln!("no syscall {} name",syscall_id);
            panic!("no syscall {} name",syscall_id)
        },
    }
}