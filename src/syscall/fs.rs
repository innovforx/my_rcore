use crate::{Errorln, print, println};

const FD_STDOUT:usize = 1;

pub fn sys_write(fd :usize,buf : *const u8,len : usize) -> isize{
    match fd {
        FD_STDOUT => {
            let slice : &[u8]= unsafe {
                core::slice::from_raw_parts(buf, len)
            };
            // let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            println!("{}",str);
            
            len as isize            
        },
        _=> {
            Errorln!("error fd");
            panic!("error fd");
        }
    }
}