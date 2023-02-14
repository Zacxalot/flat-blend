use std::ptr::null_mut;

use super::{bm_edge::BMEdge, bm_vert::BMVert};

pub struct BMDiskLink {
    next: *mut BMEdge,
    prev: *mut BMEdge,
}

impl BMDiskLink {
    pub fn new() -> Self {
        BMDiskLink {
            next: null_mut(),
            prev: null_mut(),
        }
    }
}

pub fn bmesh_disk_edge_append(e: &mut BMEdge, v: *mut BMVert) {
    unsafe {
        if (*v).edge.is_none() {
            let dl1 = bmesh_disk_edge_link_from_vert(e, v);
            (*v).edge = Some(e);
            (*dl1).next = e;
            (*dl1).prev = e;
        } else {
        }
    }
}

pub fn bmesh_disk_edge_link_from_vert(e: *mut BMEdge, v: *mut BMVert) -> *mut BMDiskLink {
    unsafe {
        if (*e).v0 == v {
            (*e).v0_disk_link = BMDiskLink::new();
            &mut ((*e).v0_disk_link)
        } else {
            (*e).v1_disk_link = BMDiskLink::new();
            &mut ((*e).v1_disk_link)
        }
    }
}
