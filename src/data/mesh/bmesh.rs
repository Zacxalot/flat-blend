use std::mem::ManuallyDrop;

use super::{bm_edge::BMEdge, bm_vert::BMVert};

pub struct BMesh {
    pub vertices: Vec<ManuallyDrop<BMVert>>,
    pub edges: Vec<ManuallyDrop<BMEdge>>,
    pub loops: Vec<ManuallyDrop<BMEdge>>,
    pub faces: Vec<ManuallyDrop<BMEdge>>,
}

impl BMesh {
    pub fn new() -> BMesh {
        let vertices = vec![];
        let edges = vec![];
        let loops = vec![];
        let faces = vec![];

        BMesh {
            vertices,
            edges,
            loops,
            faces,
        }
    }
}
