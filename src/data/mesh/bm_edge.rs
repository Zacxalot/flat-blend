use std::mem::ManuallyDrop;

use super::{bm_disk_link::BMDiskLink, bm_loop::BMLoop, bm_vert::BMVert};

#[derive(Debug)]
pub struct BMEdge {
    pub v0: *mut ManuallyDrop<BMVert>,
    pub v1: *mut ManuallyDrop<BMVert>,
    pub r#loop: *mut ManuallyDrop<BMLoop>,
    pub v1_disk_link: *mut ManuallyDrop<BMDiskLink>,
    pub v2_disk_link: *mut ManuallyDrop<BMDiskLink>,
}
