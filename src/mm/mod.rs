use self::memory_set::KERNEL_SPACE;






mod heap_allocator;
mod memory_set;
mod page_table;
mod frame_allocator;
mod address;

pub fn init(){
    heap_allocator::init_heap();
    frame_allocator::init_frame_alloctor();
    KERNEL_SPACE.exclusive_access().activate();
}