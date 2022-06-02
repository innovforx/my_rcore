

use alloc::vec::Vec;
use bitflags::*;
use super::address::*;
use super::frame_allocator::FrameTracker;


bitflags! {
    pub struct PTEFlages : u8{
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}


#[derive(Clone, Copy)]
#[repr(C)]
pub struct PageTableEntry{
    pub bits : usize,
}
/// 格式： RESERVED(10) + PPN（44）+ RSW(2) + FLAG(8)
impl PageTableEntry {
    pub fn new(ppn : PhysPageNum,flags : PTEFlages) -> Self{
        PageTableEntry { bits: ppn.0 << 10 | flags.bits as usize }
    }

    pub fn empty() -> Self{
        PageTableEntry { bits: (0) }
    }


    pub fn ppn(&self) -> PhysPageNum{
        ((self.bits >> 10) & ((1 << PPN_WIDTH_SV39) - 1)).into()
    }

    pub fn flags(&self) -> PTEFlages{
        //直接截取
        PTEFlages::from_bits(self.bits as u8).unwrap()
    }

    pub fn is_valid(&self) -> bool{
        (self.flags() & PTEFlages::V) != PTEFlages::empty()
    }

    pub fn readable(&self) -> bool{
        (self.flags() & PTEFlages::R) != PTEFlages::empty()
    }

    pub fn writable(&self) -> bool{
        (self.flags() & PTEFlages::W) != PTEFlages::empty()
    }

    pub fn executable(&self) -> bool{
        (self.flags() & PTEFlages::X) != PTEFlages::empty()
    }

}


pub struct PageTable{
    root_ppn : PhysPageNum,
    frames : Vec<FrameTracker>,
}
