mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod paging;
mod uaccess;

pub use address::{PhysAddr, VirtAddr};
pub use frame_allocator::PhysFrame;
pub use memory_set::{kernel_aspace, MapArea, MemorySet};
pub use paging::{GenericPTE, PageTableImpl};
pub use uaccess::{UserInOutPtr, UserInPtr, UserOutPtr};

pub const PAGE_SIZE: usize = 0x1000;

bitflags::bitflags! {
    pub struct MemFlags: usize {
        const READ          = 1 << 0;
        const WRITE         = 1 << 1;
        const EXECUTE       = 1 << 2;
        const USER          = 1 << 3;
        const DEVICE        = 1 << 4;
    }
}

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    memory_set::init_kernel_aspace();
}
