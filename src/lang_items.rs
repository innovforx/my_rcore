
use core::panic::PanicInfo;

use core::arch::asm;
use core::ptr;
use crate::sbi::shutdown;
use crate::{println, Infoln};


unsafe fn backtrace(){
    let mut fp : *const usize;
    asm!(
        "mv {}, fp",
        out(reg) fp,
    );
    Infoln!("backtrace");
    while fp != ptr::null() {
        let ra = *fp.sub(1); //fp寄存器所指向的位置偏移-4就是上一级返回地址的存储地址
        let fp_t = *fp.sub(2);//fp 寄存器所指向的位置偏移-8就是上一级fp的存储地址

        println!("0x{:016x}, fp = 0x{:016x}", ra, fp_t);

        fp = fp_t as *const usize;

    }
    Infoln!("end backtrace");

}


#[panic_handler]
fn panic(info : &PanicInfo)->!{

    if let Some(location) = info.location(){
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("Panicked at {}",info.message().unwrap());
    };
    
    unsafe{
        backtrace();
    }
    
    shutdown();
}