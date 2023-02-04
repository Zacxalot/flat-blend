use std::{cell::RefCell, mem::ManuallyDrop, rc::Rc};

use crate::data::{bm_edge::BMEdge, bm_vert::BMVert};

pub struct BMLoop {
    pub vertex: Rc<RefCell<BMVert>>,
    pub edge: Rc<RefCell<BMEdge>>,
    pub face: Rc<RefCell<EFace>>,
    pub next: Option<Rc<RefCell<BMLoop>>>,
    pub prev: Option<Rc<RefCell<BMLoop>>>,
}

pub struct EFace {
    loop_start: Rc<RefCell<BMLoop>>,
    loop_len: usize,
}

pub struct EMesh {
    pub vertices: Vec<ManuallyDrop<BMVert>>,
    pub edges: Vec<ManuallyDrop<BMEdge>>,
    // loops: Vec<Rc<RefCell<ELoop>>>,
    // faces: Vec<Rc<RefCell<EFace>>>,
}

pub fn gen_square() -> EMesh {
    let mut vertices = vec![
        ManuallyDrop::new(BMVert::from((-1.0, -1.0))),
        ManuallyDrop::new(BMVert::from((-1.0, 1.0))),
        ManuallyDrop::new(BMVert::from((1.0, 1.0))),
        ManuallyDrop::new(BMVert::from((1.0, -1.0))),
    ];

    let mut edges = vec![
        ManuallyDrop::new(BMEdge {
            v0: &mut vertices[0],
            v1: &mut vertices[1],
        }),
        ManuallyDrop::new(BMEdge {
            v0: &mut vertices[1],
            v1: &mut vertices[2],
        }),
        ManuallyDrop::new(BMEdge {
            v0: &mut vertices[2],
            v1: &mut vertices[3],
        }),
        ManuallyDrop::new(BMEdge {
            v0: &mut vertices[3],
            v1: &mut vertices[0],
        }),
    ];

    vertices[0].edge = Some(&mut edges[0]);
    vertices[1].edge = Some(&mut edges[1]);
    vertices[2].edge = Some(&mut edges[2]);
    vertices[3].edge = Some(&mut edges[3]);

    EMesh {
        vertices,
        edges,
        // loops,
        // faces,
    }
}
