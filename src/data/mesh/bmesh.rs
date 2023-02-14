use typed_arena::Arena;

use super::{bm_edge::BMEdge, bm_face::BMFace, bm_loop::BMLoop, bm_vert::BMVert};

pub struct BMesh {
    pub vertices: Arena<BMVert>,
    pub edges: Arena<BMEdge>,
    pub loops: Arena<BMLoop>,
    pub faces: Arena<BMFace>,
}

impl BMesh {
    pub fn new() -> BMesh {
        let vertices = Arena::new();
        let edges = Arena::new();
        let loops = Arena::new();
        let faces = Arena::new();

        BMesh {
            vertices,
            edges,
            loops,
            faces,
        }
    }
}
