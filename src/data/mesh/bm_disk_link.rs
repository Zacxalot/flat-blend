// use super::e_edge::EEdgeRc;

use std::mem::ManuallyDrop;

use super::bm_edge::BMEdge;

pub struct BMDiskLink {
    next: *mut ManuallyDrop<BMEdge>,
    prev: *mut ManuallyDrop<BMEdge>,
}
