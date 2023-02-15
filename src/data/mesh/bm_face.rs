use std::mem::ManuallyDrop;

use super::{bm_loop::BMLoop, bmesh::BMesh};

pub struct BMFace {
    loop_start: *mut ManuallyDrop<BMLoop>,
    loop_len: usize,
}

pub fn bm_face_kill(bmesh: &mut BMesh, face: *mut BMFace) {}
