use std::mem::ManuallyDrop;

use super::{
    bm_edge::{BMEdge, PBMEdge},
    bm_face::BMFace,
    bm_vert::{BMVert, PBMVert},
};

pub type PBMLoop = *mut ManuallyDrop<BMLoop>;

pub struct BMLoop {
    pub vertex: PBMVert,
    pub edge: PBMEdge,
    pub face: *mut ManuallyDrop<BMFace>,
    pub next: Option<PBMLoop>,
    pub prev: Option<PBMLoop>,
    pub radial_next: Option<PBMLoop>,
    pub radial_prev: Option<PBMLoop>,
}
