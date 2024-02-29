use core::arch::asm;

use alloc::{collections::BTreeMap, vec::Vec};
use bitflags::bitflags;
use riscv::register::satp;

use crate::{config::PAGE_SIZE, mm::address::StepByOne};

use super::{address::{PhysPageNum, VirtAddr, VirtPageNum}, frame_allocator::{frame_alloc, FrameTracker}, page_table::{PTEFlags, PageTable, PageTableEntry}, VPNRange};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MapType {
    Indentical,
    Framed,
    Linear(isize),
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MapPermission: u8 {
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}

pub struct MapArea {
    vpn_range: VPNRange,
    data_frame: BTreeMap<VirtPageNum, FrameTracker>,
    map_type: MapType,
    map_perm: MapPermission,
}

impl MapArea {
    pub fn new(start_va: VirtAddr, end_va: VirtAddr, map_type: MapType, map_perm: MapPermission) -> Self {
        let start_vpn = start_va.floor();
        let end_vpn = end_va.ceil();

        Self {
            vpn_range: VPNRange::new(start_vpn, end_vpn),
            data_frame: BTreeMap::new(),
            map_type,
            map_perm,
        }
    }

    pub fn from_another(another: &MapArea) -> Self {
        Self {
            vpn_range: VPNRange::new(another.vpn_range.get_start(), another.vpn_range.get_end()),
            data_frame: BTreeMap::new(),
            map_type: another.map_type,
            map_perm: another.map_perm,
        }
    }

    pub fn map_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        let ppn: PhysPageNum = match self.map_type {
            MapType::Indentical => {
                PhysPageNum(vpn.0)
            }
            MapType::Framed => {
                let frame = frame_alloc().unwrap();
                let frame_ppn = frame.ppn;
                self.data_frame.insert(vpn, frame);
                frame_ppn
            }
            MapType::Linear(pn_offset) => {
                assert!(vpn.0 < (1usize << 27));
                PhysPageNum((vpn.0 as isize + pn_offset) as usize)
            }
        };
        let pte_flags = PTEFlags::from_bits(self.map_perm.bits().into()).unwrap();
        page_table.map(vpn, ppn, pte_flags);
    }

    pub fn unmap_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        if self.map_type == MapType::Framed {
            self.data_frame.remove(&vpn);
        }
        page_table.unmap(vpn);
    }

    pub fn map(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.map_one(page_table, vpn);
        }
    }

    pub fn unmap(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.unmap_one(page_table, vpn);
        }
    }

    pub fn copy_data(&mut self, page_table: &mut PageTable, data: &[u8]) {
        assert_eq!(self.map_type, MapType::Framed);

        let mut start: usize = 0;
        let mut current_vpn = self.vpn_range.get_start();
        let len = data.len();

        loop {
            let src = &data[start..len.min(start + PAGE_SIZE)];
            let dst = &mut page_table
                .translate(current_vpn)
                .unwrap()
                .ppn()
                .get_byte_array()[..src.len()];
            dst.copy_from_slice(src);
            start += PAGE_SIZE;
            if start >= len {
                break;
            }
            current_vpn.step();
        }
    }
}

pub struct MemorySet {
    page_table: PageTable,
    areas: Vec<MapArea>,
}

impl MemorySet {
    pub fn new() -> Self {
        Self {
            page_table: PageTable::new(),
            areas: Vec::new(),
        }
    }

    pub fn satp(&self) -> usize {
        self.page_table.satp()
    }

    pub fn push(&mut self, mut map_area: MapArea, data: Option<&[u8]>) {
        map_area.map(&mut self.page_table);
        if let Some(data) = data {
            map_area.copy_data(&mut self.page_table, data);
        }
        self.areas.push(map_area);
    }

    pub fn insert_framed_area(&mut self, start_va: VirtAddr, end_va: VirtAddr, map_perm: MapPermission) {
        self.push(MapArea::new(start_va, end_va, MapType::Framed, map_perm), None);
    }

    pub fn remove_area_with_start_vpn(&mut self, start_vpn: VirtPageNum) {
        if let Some((idx, area)) = self.areas.iter_mut().enumerate().find(|(_, area)| area.vpn_range.get_start() == start_vpn) {
            area.unmap(&mut self.page_table);
            self.areas.remove(idx);
        }
    }

    pub fn activate(&self) {
        let satp = self.page_table.satp();
        unsafe {
            satp::write(satp);
            asm!("sfence.vma");
        }
    }

    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.page_table.translate(vpn)
    }

    pub fn recycle_data_pages(&mut self) {
        self.areas.clear();
    }
}
