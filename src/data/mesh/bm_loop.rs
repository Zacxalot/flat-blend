use std::mem::ManuallyDrop;

use super::{bm_edge::BMEdge, bm_face::BMFace, bm_vert::BMVert};

pub struct BMLoop {
    pub vertex: *mut ManuallyDrop<BMVert>,
    pub edge: *mut ManuallyDrop<BMEdge>,
    pub face: *mut ManuallyDrop<BMFace>,
    pub next: Option<*mut ManuallyDrop<BMLoop>>,
    pub prev: Option<*mut ManuallyDrop<BMLoop>>,
    pub radial_next: Option<*mut ManuallyDrop<BMLoop>>,
    pub radial_prev: Option<*mut ManuallyDrop<BMLoop>>,
}
