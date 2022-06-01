pub fn get_num_app() -> usize{
    extern "C"{
        fn _num_app();
    }
    unsafe {
        (_num_app as *const usize).read_volatile()    
    }    
}

///读取app的ELF数据
pub fn get_app_data(app_id : usize) -> &'static [u8]{
    extern "C"{
        fn _num_app();
    }
    let num_app_ptr = _num_app as *mut usize;
    let num_app = unsafe{num_app_ptr.read_volatile()};
    let app_data_ptr = unsafe {num_app_ptr.add(1 + app_id) as *mut u8};
    let mut app_data_end_ptr = unsafe {num_app_ptr.add(1 + app_id + 1) as *mut u8};
    unsafe{
        core::slice::from_raw_parts_mut(app_data_ptr, app_data_end_ptr as usize - app_data_ptr as usize)
    }
}