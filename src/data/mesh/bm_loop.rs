use std::mem::ManuallyDrop;

use super::{bm_edge::BMEdge, bm_face::BMFace, bm_vert::BMVert};

pub type PBMLoop = *mut ManuallyDrop<BMLoop>;

pub struct BMLoop {
    pub vertex: *mut BMVert,
    pub edge: *mut BMEdge,
    pub face: *mut BMFace,
    pub next: Option<*mut BMLoop>,
    pub prev: Option<*mut BMLoop>,
    pub radial_next: Option<*mut BMLoop>,
    pub radial_prev: Option<*mut BMLoop>,
}
