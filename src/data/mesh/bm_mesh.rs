use std::{cell::RefCell, mem::ManuallyDrop, rc::Rc};

use crate::data::{bm_edge::BMEdge, bm_vert::BMVert};

pub struct EMesh {
    pub vertices: Vec<ManuallyDrop<BMVert>>,
    pub edges: Vec<ManuallyDrop<BMEdge>>,
    // loops: Vec<Rc<RefCell<ELoop>>>,
    // faces: Vec<Rc<RefCell<EFace>>>,
}

pub fn gen_square() -> EMesh {
    let mut vertices = vec![];
    let mut edges = vec![];

    EMesh {
        vertices,
        edges,
        // loops,
        // faces,
    }
}
