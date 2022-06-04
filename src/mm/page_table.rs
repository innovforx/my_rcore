

use alloc::vec::*;
use bitflags::*;
use super::address::*;
use super::frame_allocator::{FrameTracker, frame_alloc};
use alloc::vec;

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

impl PageTable {
    pub fn new() -> Self{
        let frame = frame_alloc().unwrap();
        PageTable { root_ppn: (frame.ppn), frames: vec![frame] }
    }

    pub fn from_token(satp : usize) -> Self{
        //satp 的低44位是 ppn
        Self { 
            root_ppn: PhysPageNum::from(satp & ((1usize << 44) - 1)), 
            frames: Vec::new() }
    }

    pub fn find_pte_create(&mut self, vpn : VirtPageNum) -> Option<&mut PageTableEntry>{
        let idxs = vpn.indexs();
        let mut ppn = self.root_ppn;
        let mut result : Option<&mut PageTableEntry> = None;
        for (i,idx) in idxs.iter().enumerate(){
            let pte  = &mut ppn.get_pte_array()[*idx];
            if i == 2{
                result = Some(pte);
                break;
            }
            if !pte.is_valid(){
                let frame = frame_alloc().unwrap();
                *pte = PageTableEntry::new(frame.ppn, PTEFlages::V);
                self.frames.push(frame);
            }
            ppn = pte.ppn();
        }
        result
    }


    pub fn find_pte(&mut self, vpn : VirtPageNum) -> Option<&mut PageTableEntry>{
        let idxs = vpn.indexs();
        let mut ppn = self.root_ppn;
        let mut result : Option<&mut PageTableEntry>= None;
        for (i,idx) in idxs.iter().enumerate(){
            let pte =&mut ppn.get_pte_array()[*idx];
            if i == 2{
                result = Some(pte);
                break;
            }
            if !pte.is_valid(){
                break;
            }

        }
        result
    }

    pub fn map(&mut self,vpn : VirtPageNum, ppn : PhysPageNum,flags : PTEFlages){
        let pte = self.find_pte_create(vpn).unwrap();
        *pte = PageTableEntry::new(ppn, flags | PTEFlages::V);
    }

    pub fn unmap(&mut self,vpn : VirtPageNum){
        let pte = self.find_pte(vpn).unwrap();

        *pte = PageTableEntry::empty();
    }

    pub fn translate(&self, vpn : VirtPageNum ) -> Option<PageTableEntry>{
        self.find_pte(vpn).map(|pte| *pte)
    }

    pub fn token(&self) -> usize{
        8usize<< 60 | self.root_ppn.0
    }

}

///读一个虚拟地址实际映射的物理地址的内容 ???
pub fn translate_byte_buffer(token : usize,ptr : *const u8,len : usize) -> Vec<&'static mut[u8]>{
    let pagetable = PageTable::from_token(token);
    let mut start = ptr as usize;
    let end = start + len;
    let mut v = Vec::new();
    while start < end {
        let start_va = VirtAddr::from(start);
        let mut vpn = start_va.floor();
        let ppn = pagetable.translate(vpn).unwrap().ppn();
        vpn.step();
        let mut end_va : VirtAddr = vpn.into();    
        end_va = end_va.min(VirtAddr::from(end));
        if end_va.page_offset() == 0{
            v.push(&mut ppn.get_bytes_array()[start_va.page_offset()..]);
        }else{
            v.push(&mut ppn.get_bytes_array()[start_va.page_offset()..end_va.page_offset()]);
        }
    }
    v
}