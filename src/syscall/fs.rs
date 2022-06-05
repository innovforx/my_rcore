use crate::{Errorln, print, println, task::current_app_token ,mm::translate_byte_buffer};

const FD_STDOUT:usize = 1;

pub fn sys_write(fd :usize,buf : *const u8,len : usize) -> isize{
    match fd {
        FD_STDOUT => {

            let buffers = translate_byte_buffer(current_app_token(),buf,len);

            for buffer in buffers {
                print!("{}", core::str::from_utf8(buffer).unwrap());
            }
            len as isize        
        },
        _=> {
            Errorln!("error fd");
            panic!("error fd");
        }
    }
}