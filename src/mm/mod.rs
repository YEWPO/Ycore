mod address;
mod page_table;
mod heap_allocator;

pub fn init() {
    heap_allocator::init_heap();
}
