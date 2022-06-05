pub const USER_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;
pub const MEMORY_END:usize = 0x8080_0000;
pub const PAGE_SIZE:usize = 0x1_000;
pub const PAGE_SIZE_BITS : usize = 12usize;

pub const TRAMPLINE : usize = usize::MAX - (PAGE_SIZE - 1);
pub const TRAP_CONTEXT :usize = TRAMPLINE - PAGE_SIZE;

pub fn kernel_stack_position(appid : usize) -> (usize,usize){
    let top = TRAMPLINE - appid*(KERNEL_STACK_SIZE + PAGE_SIZE);
    let bottom = top - KERNEL_STACK_SIZE;
    (bottom,top)
}


pub use crate::board::CLOCK_FREQ;