mod address;
mod page_table;
mod heap_allocator;
mod frame_allocator;
mod memory_set;

pub use address::{VPNRange, PPNRange};

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
}
