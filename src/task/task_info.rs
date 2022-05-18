
use core::cell::*;

// mod task;
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Task_info{
    pub name : Option<[u8;20]>,    
}

impl  Task_info {
    pub fn new(name : [u8;20]) -> Self{        
        Task_info{name : Some(name)}
    }
}