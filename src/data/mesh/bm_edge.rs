use std::{ptr::null_mut};

use super::{
    bm_disk_link::{bmesh_disk_edge_append, BMDiskLink},
    bm_loop::BMLoop,
    bm_vert::BMVert,
    bmesh::BMesh,
};

pub struct BMEdge {
    pub v0: *mut BMVert,
    pub v1: *mut BMVert,
    pub r#loop: *mut BMLoop,
    pub v0_disk_link: BMDiskLink,
    pub v1_disk_link: BMDiskLink,
}

pub fn bm_edge_create(bmesh: &mut BMesh, v0: *mut BMVert, v1: *mut BMVert) -> *mut BMEdge {
    let mut e = bmesh.edges.alloc(BMEdge {
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
