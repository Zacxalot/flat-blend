use std::ptr::null_mut;

use super::{
    bm_disk_link::{bmesh_disk_edge_append, bmesh_disk_edge_remove, BMDiskLink},
    bm_face::bm_face_kill,
    bm_loop::BMLoop,
    bm_vert::BMVert,
    bmesh::BMesh,
};

pub struct BMEdge {
    pub v0: *mut BMVert,
    pub v1: *mut BMVert,
    pub r#loop: Option<*mut BMLoop>,
    pub v0_disk_link: BMDiskLink,
    pub v1_disk_link: BMDiskLink,
}

pub fn bm_edge_create(bmesh: &mut BMesh, v0: *mut BMVert, v1: *mut BMVert) -> *mut BMEdge {
    let e = bmesh.edges.alloc(BMEdge {
        v0,
        v1,
        r#loop: None,
        v0_disk_link: BMDiskLink::new(),
        v1_disk_link: BMDiskLink::new(),
    });

    bmesh_disk_edge_append(e, v0);
    bmesh_disk_edge_append(e, v1);

    e
}

pub fn bm_edge_kill(bmesh: &mut BMesh, edge: *mut BMEdge) {
    unsafe {
        while let Some(r#loop) = (*edge).r#loop {
            bm_face_kill(bmesh, (*r#loop).face);
        }

        bmesh_disk_edge_remove(edge, (*edge).v0);
        bmesh_disk_edge_remove(edge, (*edge).v1);
    }
}

pub fn bm_kill_only_edge(bmesh: &mut BMesh, edge: *mut BMEdge) {}
