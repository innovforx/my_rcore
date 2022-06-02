use alloc::vec::Vec;

use crate::{sync::UPSafeCell, mm::address::PhyAddr, println};

use super::address::PhysPageNum;
use core::fmt::{self, Debug, Formatter};
use lazy_static::lazy_static;
use crate::config::MEMORY_END;

trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn dealloc(&mut self,ppn : PhysPageNum); 
}


pub struct StackFrameAllocator{
    current : usize,
    end : usize,
    recycled : Vec<usize>,
}


impl StackFrameAllocator {
    pub fn init(&mut self,l : PhysPageNum, r : PhysPageNum){
        self.current = l.0;
        self.end = r.0;
    }
}


impl FrameAllocator for StackFrameAllocator {
    fn new() -> Self {
        Self { current: (0), end: (0), recycled: (Vec::new()) }
    }
    fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(ppn) = self.recycled.pop(){
            Some(ppn.into())
        }else if self.current == self.end{
            None
        }else {
            self.current += 1;
            Some((self.current - 1).into())
        }
    }

    fn dealloc(&mut self, ppn : PhysPageNum){
        let ppn = ppn.0;
        if ppn >= self.current || self.recycled.iter().any(| &v | v == ppn){            
            return ;
        }
        self.recycled.push(ppn);
    }
}


type FrameAllocatorImpl = StackFrameAllocator;

lazy_static!{
    pub static ref FRAME_ALLOCATOR : UPSafeCell<FrameAllocatorImpl> = unsafe{
        UPSafeCell::new(FrameAllocator::new())
    };
}

pub fn init_frame_alloctor(){
    extern "C"{
        fn ekernel();
    }
    FRAME_ALLOCATOR.exclusive_access().init(
        PhyAddr::from(ekernel as usize).ceil(), 
        PhyAddr::from(MEMORY_END as usize).floor());
}

pub fn frame_alloc() -> Option<FrameTracker>{
    FRAME_ALLOCATOR
        .exclusive_access()
        .alloc()
        .map(FrameTracker::new)
}

pub fn frame_dealloc(ppn : PhysPageNum){
    FRAME_ALLOCATOR
    .exclusive_access()
    .dealloc(ppn)
}



pub struct FrameTracker{
    pub ppn : PhysPageNum,
}

impl FrameTracker {
    pub fn new(ppn : PhysPageNum) -> Self{
        let buf = ppn.get_bytes_array();
        for i in buf{
            *i = 0;
        }
        Self { ppn: (ppn) }
    }
}

impl Debug for FrameTracker {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("FrameTracker:PPN={:#x}", self.ppn.0))
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        frame_dealloc(self.ppn);
    }
}

#[allow(unused)]
pub fn frame_alloc_test(){
    let mut v :Vec<FrameTracker> = Vec::new();

    for i in 0..5 {
        let frame = frame_alloc().unwrap();
        println!("alloc {:?}",frame);
        v.push(frame);
    }
    v.clear();

    for i in 0..5{
        let frame = frame_alloc().unwrap();
        println!("{:?}",frame);
        v.push(frame);
    }
    drop(v);
    println!("test frame allocator success");
}

