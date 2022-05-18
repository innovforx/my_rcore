use core::arch::asm;
// use core::panicking::panic;
use crate::sync::UPSafeCell;
use crate::Infoln;
use lazy_static::*;
use crate::trap::TrapContext;


const USER_STACK_SIZE:usize = 4096*2;
const KERNEL_STACK_SIZE : usize = 4096*2;


const MAX_APP_NUM :usize = 6;
const APP_BASE_ADDRESS:usize = 0x80400000;
const APP_SIZE_LIMIT:usize = 0x20000;


#[repr(align(4096))]
struct KernelStack{
    data:[u8;KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
struct UserStack{
    data:[u8;USER_STACK_SIZE],
}

static KERNEL_STACK:KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};

static USER_STACK:KernelStack = KernelStack {
    data: [0; USER_STACK_SIZE],
};


impl KernelStack {
    fn get_sp(&self) -> usize{
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    pub fn push_context(&self,cx : TrapContext) -> &'static mut TrapContext{
        //相当于入栈了
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>())as *mut TrapContext;
        unsafe{
            *cx_ptr = cx;   //move
        }
        unsafe{ cx_ptr.as_mut().unwrap() }
    }
    
}

impl UserStack {
    fn get_sp(&self) -> usize{
        //向下增长的，所以要加上USER_STACK_SIZE
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}


struct AppManager{
    num_app : usize,
    current_app :usize,
    app_start:[usize;MAX_APP_NUM + 1],
}

impl AppManager {
    
    pub fn print_app_info(&self){
        Infoln!("[kernel] num_app = {}",self.num_app);
        for i in 0..self.num_app{
            Infoln!(
                "Kernel app_{}:[{:#x} - {:#x}]"
                ,&i
                ,self.app_start[i],
                self.app_start[i+1]
            );
        }
    }

    unsafe fn load_app(&self,app_id:usize){
        if app_id >= self.num_app{
            panic!("app id error");
        }
        Infoln!("Kernel Loading app{}",app_id);
        asm!("fence.i");
        //因为要写0 所以是mut
        core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);

        // from_raw_parts 是 将指定的地址构造成 不可变slice
        let app_src = core::slice::from_raw_parts(
            self.app_start[app_id] as *const u8,
            self.app_start[app_id +1] - self.app_start[app_id]);
        
        
        let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());

        app_dst.clone_from_slice(app_src);

    }

    pub fn get_current_app(&self)->usize{
        self.current_app
    }

    pub fn move_to_next_app(&mut self){
        self.current_app += 1;
    }
}

lazy_static!{
    static ref APP_MENAGER : UPSafeCell<AppManager> = unsafe{
        UPSafeCell::new({
            extern "C" {
                fn _num_app();
            }
            let num_app_ptr = _num_app as usize as *mut usize;
            let num_app = num_app_ptr.read_volatile();

            let mut app_start : [usize;MAX_APP_NUM + 1] = [0;MAX_APP_NUM + 1];

            let app_start_raw_mut : &[usize]= core::slice::from_raw_parts_mut(num_app_ptr.add(1), num_app + 1);

            app_start[..=num_app].copy_from_slice(app_start_raw_mut);

            AppManager { num_app: (num_app), current_app: (0), app_start: (app_start) }

        })
    };
}


pub fn init(){
    APP_MENAGER.exclusive_access().print_app_info();
}


pub fn run_next_app() -> ! {
    let mut app_mamger = APP_MENAGER.exclusive_access();
    let curent_app = app_mamger.current_app;

    unsafe{
        app_mamger.load_app(curent_app);
    }

    app_mamger.move_to_next_app();

    drop(app_mamger);


    extern "C"{
        fn __restore(cx_addr : usize);
    }

    unsafe{
        __restore(KERNEL_STACK.push_context(
            TrapContext
                ::app_init_context(APP_BASE_ADDRESS, USER_STACK.get_sp()
                )
            )as *const _ as usize            
        );
    }

    panic!("Unreachable in batch::run_current_app!");

}
