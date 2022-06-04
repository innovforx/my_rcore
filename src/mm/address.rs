
use core::mem::size_of;

use crate::config::PAGE_SIZE_BITS;
use crate::config::PAGE_SIZE;
use super::page_table::*;
pub const PA_WIDTH_SV39 : usize = 56;
pub const VA_WIDTH_SV39 : usize = 39;
pub const PPN_WIDTH_SV39 : usize = PA_WIDTH_SV39 - PAGE_SIZE_BITS;
pub const VPN_WIDTH_SV39 : usize = VA_WIDTH_SV39 - PAGE_SIZE_BITS;


#[derive(Clone, Copy,PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PhyAddr(pub usize);

#[derive(Clone, Copy,PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct VirtAddr(pub usize);

#[derive(Clone, Copy,PartialEq, Eq, PartialOrd, Ord,Debug)]
pub struct PhysPageNum(pub usize);

#[derive(Clone, Copy,PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct VirtPageNum(pub usize);

impl From<usize> for PhyAddr {
    fn from( v: usize) -> Self {
        PhyAddr(v & ((1 << PA_WIDTH_SV39) - 1))
    }
}

impl From<usize> for VirtAddr {
    fn from(v: usize) -> Self {
        VirtAddr(v & ((1 << VA_WIDTH_SV39) - 1))
    }
}

impl From<usize> for PhysPageNum {
    fn from(v: usize) -> Self {
        PhysPageNum(v & ((1 << PPN_WIDTH_SV39) - 1))
    }
}

impl From<usize> for VirtPageNum {
    fn from(v: usize) -> Self {
        VirtPageNum( v & ((1 << VPN_WIDTH_SV39) - 1))
    }
}


impl From<PhyAddr> for usize {
    fn from( pa: PhyAddr) -> Self {
        pa.0
    }
}


impl From<VirtAddr> for usize {
    fn from( va: VirtAddr) -> Self {
        va.0
    }
}

impl From<PhysPageNum> for usize {
    fn from( ppn: PhysPageNum) -> Self {
        ppn.0
    }
}

impl From<VirtPageNum> for usize {
    fn from( vpn: VirtPageNum) -> Self {
        vpn.0
    }
}



impl VirtAddr {
    pub fn floor(&self) -> VirtPageNum{
        VirtPageNum((self.0 & !((PAGE_SIZE) - 1)) >> PAGE_SIZE_BITS)
    }

    pub fn ceil(&self) -> VirtPageNum{
        //-1是因为当 xx..xx 0000的时候向上取整应该不变，但是因为计算的时候会加上pagesize所以会导致计算错误，所以减一修正
        VirtPageNum( ((self.0 - 1 + PAGE_SIZE) & !((PAGE_SIZE) - 1)) >> PAGE_SIZE_BITS )
    }

    pub fn page_offset(&self) -> usize{
        self.0 & (PAGE_SIZE - 1)
    }

    pub fn aligned(&self) -> bool{
        self.page_offset() == 0
    }
}

impl From<VirtAddr> for VirtPageNum {
    fn from( va: VirtAddr) -> Self {
        va.floor()
    }

}

impl From<VirtPageNum> for VirtAddr {
    fn from( vpn: VirtPageNum) -> Self {
        Self(vpn.0 << PAGE_SIZE_BITS)       
    }
}


impl PhyAddr {
    pub fn floor(&self) -> PhysPageNum{
        PhysPageNum((self.0 & !((1 << PAGE_SIZE_BITS) - 1)) >> PAGE_SIZE_BITS)
    }

    pub fn ceil(&self) -> PhysPageNum{
        PhysPageNum(((self.0 - 1 + PAGE_SIZE ) & !((1 << PAGE_SIZE_BITS) - 1)) >> PAGE_SIZE_BITS )
    }

    pub fn page_offset(&self) -> usize{
        self.0 & (PAGE_SIZE - 1)
    }

    pub fn aligned(&self) -> bool{
        self.page_offset() == 0
    }

}

impl From<PhyAddr> for PhysPageNum {
    fn from(pa: PhyAddr) -> Self {
        Self( pa.0 & (PAGE_SIZE - 1) )
    }
}

impl From<PhysPageNum> for PhyAddr {
    fn from( ppn: PhysPageNum) -> Self {
        Self( ppn.0 << PAGE_SIZE_BITS )
    }
}

impl VirtPageNum {
    pub fn indexs(&self) -> [usize;3]{
        let mut vpn = self.0;
        let mut idx = [0usize;3];
        for i in (0..3).rev(){
            idx[i] = vpn & ((1 << 9)-1);
            vpn >>= 9;
        }
        idx
    }
}

impl PhysPageNum {
    pub fn get_pte_array(&self) -> &'static mut[PageTableEntry]{
        let pa : PhyAddr = (*self).into();
        unsafe{
            core::slice::from_raw_parts_mut(pa.0 as *mut PageTableEntry, PAGE_SIZE/size_of::<PageTableEntry>() )
        }
    }

    pub fn get_bytes_array(&self) -> &'static mut [u8]{
        let pa : PhyAddr = (*self).into();
        unsafe {
            core::slice::from_raw_parts_mut(pa.0 as *mut u8, PAGE_SIZE / size_of::<u8>())
        }
    }

    pub fn get_mut<T>(&self) -> &'static mut T{
        let pa : PhyAddr = (*self).into();
        unsafe{
            (pa.0 as *mut T).as_mut().unwrap()
        }
    }
}

pub trait StepByOne {
    fn step(&mut self);
}

impl StepByOne for VirtPageNum {
    fn step(&mut self) {
        self.0 += 1;
    }
}

use core::fmt::{Debug};

#[derive(Clone, Copy)]
pub struct SimpleRange<T>
where
    T : StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    l : T,
    r : T,
}


impl <T> SimpleRange<T>
where
    T : StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    pub fn new(start : T,end : T) ->Self {
        Self { l: (start), r: (end) }
    }
    pub fn get_start(&self) -> T {
        self.l
    }

    pub fn get_end(&self) -> T {
        self.r
    }
}


pub struct SimpleRangeIterator<T>
where
    T : StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    current : T,
    end : T,
}


impl <T> SimpleRangeIterator<T>
where
    T : StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    pub fn new(l : T,r : T) -> Self{
        Self{
            current : l,
            end : r,
        }
    }
}


impl<T> Iterator for SimpleRangeIterator<T>
where
    T : StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end{
            None
        }else{
            let t = self.current;
            self.current.step();
            Some(t)
        }
    }
}

impl<T> IntoIterator for SimpleRange<T> 
where
    T : StepByOne + Copy + PartialEq +PartialOrd + Debug,
{
    type Item = T;
    type IntoIter = SimpleRangeIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
       SimpleRangeIterator::new(self.l, self.r)
    }
    
}

pub type VPNRange = SimpleRange<VirtPageNum>;
