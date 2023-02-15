use std::ptr::null_mut;

use super::{bm_edge::BMEdge, bm_vert::BMVert};

#[derive(Debug)]
pub struct BMDiskLink {
    next: Option<*mut BMEdge>,
    prev: Option<*mut BMEdge>,
}

impl BMDiskLink {
    pub fn new() -> Self {
        BMDiskLink {
            next: None,
            prev: None,
        }
    }
}

pub fn bmesh_disk_edge_append(e: &mut BMEdge, v: *mut BMVert) {
    unsafe {
        if let Some(v_edge) = (*v).edge {
            let dl1 = bmesh_disk_edge_link_from_vert(e, v);
            let dl2 = bmesh_disk_edge_link_from_vert(v_edge, v);

            (*dl1).next = Some(v_edge);
            (*dl1).prev = (*dl2).prev;

            if let Some(dl2_prev_edge) = (*dl2).prev {
                let dl3 = bmesh_disk_edge_link_from_vert(dl2_prev_edge, v);
                (*dl3).next = Some(e);
            }

            (*dl2).prev = Some(e);
        } else {
            let dl1 = bmesh_disk_edge_link_from_vert(e, v);
            (*v).edge = Some(e);
            (*dl1).next = Some(e);
            (*dl1).prev = Some(e);
        }
    }
}

pub fn bmesh_disk_edge_remove(e: *mut BMEdge, v: *mut BMVert) {
    unsafe {
        let dl1 = bmesh_disk_edge_link_from_vert(e, v);
        if let Some(dl1_prev) = (*dl1).prev {
            let dl2 = bmesh_disk_edge_link_from_vert(dl1_prev, v);
            (*dl2).next = (*dl1).next;
        }

        if let Some(dl1_next) = (*dl1).next {
            let dl2 = bmesh_disk_edge_link_from_vert(dl1_next, v);
            (*dl2).prev = (*dl1).prev;
        }

        if let Some(v_edge) = (*v).edge {
            if v_edge == e {
                if (*dl1).next != Some(e) {
                    (*v).edge = (*dl1).next;
                } else {
                    (*v).edge = None
                }
            }
        }

        (*dl1).next = None;
        (*dl1).prev = None;
    }
}

pub fn bmesh_disk_edge_link_from_vert(e: *mut BMEdge, v: *mut BMVert) -> *mut BMDiskLink {
    unsafe {
        if (*e).v0 == v {
            &mut (*e).v0_disk_link
        } else {
            &mut (*e).v1_disk_link
        }
    }
}
