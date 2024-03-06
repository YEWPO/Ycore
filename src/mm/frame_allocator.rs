use core::fmt::Debug;

use alloc::vec::Vec;
use lazy_static::lazy_static;
use log::{debug, info};

use crate::{config::MEMORY_END, mm::address::PhysAddr, sync::UPIntrFreeCell};

use super::address::PhysPageNum;

pub struct FrameTracker {
    pub ppn: PhysPageNum,
}

impl Debug for FrameTracker {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("FrameTracker: PPN = {:#x}", self.ppn.0))
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        frame_dealloc(self.ppn)
    }
}

impl FrameTracker {
    pub fn new(ppn: PhysPageNum) -> Self {
        let byte_array = ppn.get_byte_array();
        for i in byte_array {
            *i = 0;
        }

        Self {
            ppn,
        }
    }
}

trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn alloc_more(&mut self, pages: usize) -> Option<Vec<PhysPageNum>>;
    fn dealloc(&mut self, ppn: PhysPageNum);
}

pub struct StackFrameAllocator {
    current: usize,
    end: usize,
    recycled: Vec<usize>,
}

impl StackFrameAllocator {
    pub fn init(&mut self, l: PhysPageNum, r: PhysPageNum) {
        self.current = l.0;
        self.end = r.0;
    }
}

impl FrameAllocator for StackFrameAllocator {
    fn new() -> Self {
        Self {
            current: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }

    fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(ppn) = self.recycled.pop() {
            Some(ppn.into())
        } else if self.current == self.end {
            None
        } else {
            self.current += 1;
            Some((self.current - 1).into())
        }
    }

    fn alloc_more(&mut self, pages: usize) -> Option<Vec<PhysPageNum>> {
        if self.recycled.len() >= pages {
            let v = self.recycled
                .drain(0..pages)
                .map(|x| x.into())
                .collect();
            Some(v)
        } else if self.current + pages >= self.end {
            None
        } else {
            self.current += pages;
            let arr: Vec<usize> = (1..pages + 1).collect();
            let v = arr.iter().map(|x| (self.current - x).into()).collect();
            Some(v)
        }
    }

    fn dealloc(&mut self, ppn: PhysPageNum) {
        let ppn = ppn.0;
        if ppn >= self.current || self.recycled.iter().any(|&x| x == ppn) {
            panic!("Frame ppn = {:#x} has not been allocated!", ppn);
        }
        self.recycled.push(ppn);
    }
}

type FrameAllocatorImpl = StackFrameAllocator;

lazy_static! {
    pub static ref FRAME_ALLOCATOR: UPIntrFreeCell<FrameAllocatorImpl> =
        unsafe { UPIntrFreeCell::new(FrameAllocatorImpl::new()) };
}

pub fn init_frame_allocator() {
    extern "C" {
        fn ekernel();
    }

    info!("Initializing frame allocator.");

    FRAME_ALLOCATOR
        .exclusive_access()
        .init(PhysAddr::from(ekernel as usize).ceil(), PhysAddr::from(MEMORY_END).floor());

    info!("Frame range [0x{:x}000, 0x{:x}000].", PhysAddr::from(ekernel as usize).ceil().0, PhysAddr::from(MEMORY_END).floor().0);

    debug!("Initialized frame allocator.");
}

pub fn frame_alloc() -> Option<FrameTracker> {
    debug!("Allocating a frame.");

    FRAME_ALLOCATOR
        .exclusive_access()
        .alloc()
        .map(FrameTracker::new)

}

pub fn frame_alloc_more(pages: usize) -> Option<Vec<FrameTracker>> {
    debug!("Allocating {} frames.", pages);

    FRAME_ALLOCATOR
        .exclusive_access()
        .alloc_more(pages)
        .map(|x| x.iter().map(|&t| FrameTracker::new(t)).collect())
}

pub fn frame_dealloc(ppn: PhysPageNum) {
    debug!("Deallocing ppn = {:#x} frame.", ppn.0);

    FRAME_ALLOCATOR
        .exclusive_access()
        .dealloc(ppn);
}
