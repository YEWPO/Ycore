mod address;
mod page_table;
mod heap_allocator;
mod frame_allocator;
mod memory_set;

pub use address::{VPNRange, PPNRange};
pub use memory_set::KERNEL_SPACE;

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}
