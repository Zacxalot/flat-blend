use std::{mem::ManuallyDrop, ptr::null_mut};

use super::{
    bm_disk_link::{bmesh_disk_edge_append, BMDiskLink},
    bm_loop::PBMLoop,
    bm_vert::PBMVert,
    bmesh::BMesh,
};

pub type PBMEdge = *mut ManuallyDrop<BMEdge>;

pub struct BMEdge {
    pub v0: PBMVert,
    pub v1: PBMVert,
    pub r#loop: PBMLoop,
    pub v0_disk_link: BMDiskLink,
    pub v1_disk_link: BMDiskLink,
}

pub fn bm_edge_create(_bmesh: &mut BMesh, v0: PBMVert, v1: PBMVert) -> ManuallyDrop<BMEdge> {
    let mut e = ManuallyDrop::new(BMEdge {
        v0,
        v1,
        r#loop: null_mut(),
        v0_disk_link: BMDiskLink::new(),
        v1_disk_link: BMDiskLink::new(),
    });

    bmesh_disk_edge_append(&mut e, v0);
    bmesh_disk_edge_append(&mut e, v1);

    e
}
