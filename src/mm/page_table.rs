use alloc::vec::Vec;
use alloc::vec;
use bitflags::bitflags;
use log::{debug, trace, warn};

use crate::config::PPN_WIDTH;

use super::{address::{PhysPageNum, VirtPageNum}, frame_allocator::{frame_alloc, FrameTracker}};

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

#[derive(Debug, Clone, Copy)]
pub struct PageTableEntry(pub usize);

impl PageTableEntry {
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        Self((ppn.0 << FLAGS_BITS) | flags.bits())
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

pub struct PageTable {
    root_ppn: PhysPageNum,
    frames: Vec<FrameTracker>,
}

impl PageTable {
    pub fn new() -> Self {
        let frame = frame_alloc().unwrap();

        Self {
            root_ppn: frame.ppn,
            frames: vec![frame],
        }
    }
    
    pub fn from_satp(satp: usize) -> Self {
        Self {
            root_ppn: PhysPageNum::from(satp & ((1usize << 44) - 1)),
            frames: Vec::new(),
        }
    }

    fn find_pte_create(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;

        for (i, idx) in idxs.iter().enumerate() {
            let pte = &mut ppn.get_pte_array()[*idx];
            
            if i == 2 {
                debug!("vpn {:?}'s pte is {:?}", vpn , &pte);

                result = Some(pte);
                break;
            }

            if !pte.is_valid() {
                trace!("pte {:?} is invalid, alloc a frame", &pte);

                let frame = frame_alloc().unwrap();
                *pte = PageTableEntry::new(frame.ppn, PTEFlags::V);
                self.frames.push(frame)
            }

            ppn = pte.ppn();
        }

        result
    }

    fn find_pte(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;

        for (i, idx) in idxs.iter().enumerate() {
            let pte = &mut ppn.get_pte_array()[*idx];

            if i == 2 {
                debug!("vpn {:?}'s pte is {:?}", vpn , &pte);

                result = Some(pte);
                break;
            }

            if !pte.is_valid() {
                warn!("pte {:?} is invalid", &pte);

                return None;
            }

            ppn = pte.ppn();
        }

        result
    }

    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let pte = self.find_pte_create(vpn).unwrap();
        assert!(!pte.is_valid(), "vpn {:?} is mapped before mapping", vpn);

        trace!("vpn {:?} is mapped to ppn {:?}", vpn, ppn);
        *pte = PageTableEntry::new(ppn, flags);
    }

    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = self.find_pte(vpn).unwrap();
        assert!(pte.is_valid(), "vpn {:?} is unmapped before unmapping", vpn);

        trace!("vpn {:?} is unmapped", vpn);
        *pte = PageTableEntry::empty();
    }
}
