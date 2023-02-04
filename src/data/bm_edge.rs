use std::mem::ManuallyDrop;

use super::bm_vert::BMVert;

#[derive(Debug)]
pub struct BMEdge {
    pub v0: *mut ManuallyDrop<BMVert>,
    pub v1: *mut ManuallyDrop<BMVert>,
}

// impl EEdge {
//     fn new(verts: (EVertRc, EVertRc)) -> Self {
//         EEdge {
//             v0: verts.0,
//             v1: verts.1,
//         }
//     }
// }
