use std::{mem::ManuallyDrop};

use super::{bm_edge::BMEdge, bm_vert::BMVert};

pub struct EMesh {
    pub vertices: Vec<ManuallyDrop<BMVert>>,
    pub edges: Vec<ManuallyDrop<BMEdge>>,
    // loops: Vec<Rc<RefCell<ELoop>>>,
    // faces: Vec<Rc<RefCell<EFace>>>,
}

pub fn gen_square() -> EMesh {
    let vertices = vec![];
    let edges = vec![];

    EMesh {
        vertices,
        edges,
        // loops,
        // faces,
    }
}
