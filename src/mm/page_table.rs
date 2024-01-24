use bitflags::bitflags;

use crate::config::PPN_WIDTH;

use super::address::PhysPageNum;

const FLAGS_BITS: usize = 10;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct PTEFlags: usize {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
        const RESERVE_0 = 1 << 8;
        const RESERVE_1 = 1 << 9;
    }
}

#[derive(Clone, Copy)]
pub struct PageTableEntry(pub usize);

impl PageTableEntry {
    pub fn new(ppn: &PhysPageNum, flags: &PTEFlags) -> Self {
        Self((ppn.0 << FLAGS_BITS) | flags.bits() as usize)
    }

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn ppn(&self) -> PhysPageNum {
        PhysPageNum::from(self.0 >> FLAGS_BITS & ((1 << PPN_WIDTH) - 1))
    }

    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.0 & ((1 << FLAGS_BITS) - 1)).unwrap()
    }

    pub fn is_valid(&self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty()
    }

    pub fn readable(&self) -> bool {
        (self.flags() & PTEFlags::R) != PTEFlags::empty()
    }

    pub fn writable(&self) -> bool {
        (self.flags() & PTEFlags::W) != PTEFlags::empty()
    }

    pub fn executable(&self) -> bool {
        (self.flags() & PTEFlags::X) != PTEFlags::empty()
    }
}
