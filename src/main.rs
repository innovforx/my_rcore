#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
// use crate::lang_items;

extern crate alloc;

#[macro_use]
extern crate bitflags;




#[cfg(feature = "board_k210")]
#[path = "boards/k210.rs"]
mod board;
#[cfg(not(any(feature = "board_qemu")))]
#[path = "boards/qemu.rs"]
mod board;


mod config;
#[macro_use]
mod lang_items;
mod sbi;
mod mm;

mod console;
mod trap;
mod loader;
mod sync;
#[macro_use]
mod syscall;
mod task;
mod timer;
// use core::panic::PanicInfo;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));
// global_asm!(include_str!("task/switch.S"));



#[no_mangle]
pub fn rust_main()-> ! {
    clear_bss();
    println!("Hello world");
    Errorln!("Err msg test red higtlight");
    Warnln!("Warn msg test yellow default");
    Infoln!("Info msg test blue underline");
    Debugln!("Debug msg test green underline");
    Traceln!("Trace msg test gray underline");
    //test

    sys_mem_info();

    println!("ready to open memory");

    mm::init();

    println!("[kernel] mm test");

    mm::remap_test();


    trap::init();    
    Debugln!("trap init success");
    trap::enable_time_handler();
    timer::set_next_trigger();
    Debugln!("ready to run first");
    task::run_first_task();
    panic!("Shutdown");

    
}


fn clear_bss(){
    extern "C"{
        fn sbss();
        fn ebss();
    }

    // sbss ebss是两个全局符号，分别代表着bss段的起始地址和结束地址 （startAddr...endAddr）就是这一串地址
    (sbss as usize..ebss as usize).for_each(|a|{
        unsafe{
            // a 就是每个地址
            (a as *mut u8).write_volatile(0)
            
        }
    });

}


fn sys_mem_info(){
    extern "C"{
        fn BASE_ADDRESS();
        fn skernel();
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn ekernel();
    }
    Infoln!("Base Address : 0X{:X}",BASE_ADDRESS as u64);
    Infoln!("skernel Address : 0X{:X}",skernel as u64);
    Infoln!("stext Address : 0X{:X}",stext as u64);
    Infoln!("etext Address : 0X{:X}",etext as u64);
    Infoln!("srodata Address : 0X{:X}",srodata as u64);
    Infoln!("erodata Address : 0X{:X}",erodata as u64);
    Infoln!("sdata Address : 0X{:X}",sdata as u64);
    Infoln!("edata Address : 0X{:X}",edata as u64);
    Infoln!("sbss Address : 0X{:X}",sbss as u64);
    Infoln!("ebss Address : 0X{:X}",ebss as u64);
    Infoln!("ekernel Address : 0X{:X}",ekernel as u64);
}


// fn main() {
//     println!("Hello, world!");
// }
