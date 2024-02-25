use alloc::collections::BTreeMap;
use bitflags::bitflags;

use super::{address::{VirtAddr, VirtPageNum}, frame_allocator::FrameTracker, VPNRange};

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
}
