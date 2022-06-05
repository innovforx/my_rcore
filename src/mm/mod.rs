mod heap_allocator;
mod memory_set;
mod page_table;
mod frame_allocator;
mod address;

pub use address::{PhyAddr, PhysPageNum, VirtAddr, VirtPageNum};
use address::{StepByOne, VPNRange};
pub use frame_allocator::{frame_alloc, FrameTracker};
pub use memory_set::remap_test;
pub use memory_set::{MapPermission, MemorySet,KERNEL_SPACE};
pub use page_table::{translate_byte_buffer, PageTableEntry};
use page_table::{PTEFlages, PageTable};

use crate::println;

pub fn init(){
    heap_allocator::init_heap();
    println!("heap init finfish");
    heap_allocator::heap_test();
    println!("heap test finfish");
    frame_allocator::init_frame_alloctor();
    println!("frame allocotor init finfish");
    KERNEL_SPACE.exclusive_access().activate();
    println!("active satp");
}