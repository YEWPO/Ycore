const VA_WIDTH_SV39: usize = 39;
const PA_WIDTH_SV39: usize = 54;

pub const KERNEL_HEAP_SIZE: usize = 0x100_0000;

pub const PAGE_SIZE: usize = 0x1000;
pub const PAGE_SIZE_BITS: usize = 12;
pub const VA_WIDTH: usize = VA_WIDTH_SV39;
pub const PA_WIDTH: usize = PA_WIDTH_SV39;
pub const VPN_WIDTH: usize = VA_WIDTH - PAGE_SIZE_BITS;
pub const PPN_WIDTH: usize = PA_WIDTH - PAGE_SIZE_BITS;

pub const MEMORY_END: usize = 0x88000000;
