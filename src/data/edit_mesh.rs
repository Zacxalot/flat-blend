use std::{cell::RefCell, rc::Rc};

use crate::data::{e_edge::EEdge, e_vert::EVert};

use super::{e_edge::EEdgeRc, e_vert::EVertRc};

pub struct ELoop {
    pub vertex: Rc<RefCell<EVert>>,
    pub edge: Rc<RefCell<EEdge>>,
    pub face: Rc<RefCell<EFace>>,
    pub next: Option<Rc<RefCell<ELoop>>>,
    pub prev: Option<Rc<RefCell<ELoop>>>,
}

pub struct EFace {
    loop_start: Rc<RefCell<ELoop>>,
    loop_len: usize,
}

pub struct EMesh {
    pub vertices: Vec<EVertRc>,
    pub edges: Vec<EEdgeRc>,
    // loops: Vec<Rc<RefCell<ELoop>>>,
    // faces: Vec<Rc<RefCell<EFace>>>,
}

pub fn gen_square() -> EMesh {
    let vertices = vec![
        Rc::from(RefCell::from(EVert::from((-1.0, -1.0)))),
        Rc::from(RefCell::from(EVert::from((-1.0, 1.0)))),
        Rc::from(RefCell::from(EVert::from((1.0, 1.0)))),
        Rc::from(RefCell::from(EVert::from((1.0, -1.0)))),
    ];

    let edges = vec![
        Rc::from(RefCell::from(EEdge {
            v0: vertices[0].clone(),
            v1: vertices[1].clone(),
        })),
        Rc::from(RefCell::from(EEdge {
            v0: vertices[1].clone(),
            v1: vertices[2].clone(),
        })),
        Rc::from(RefCell::from(EEdge {
            v0: vertices[2].clone(),
            v1: vertices[3].clone(),
        })),
        Rc::from(RefCell::from(EEdge {
            v0: vertices[3].clone(),
            v1: vertices[0].clone(),
        })),
    ];

    (*vertices[0].clone()).borrow_mut().edge = Some(edges[0].clone());
    (*vertices[1].clone()).borrow_mut().edge = Some(edges[1].clone());
    (*vertices[2].clone()).borrow_mut().edge = Some(edges[2].clone());
    (*vertices[3].clone()).borrow_mut().edge = Some(edges[3].clone());

    // println!("{:?}", (*a).borrow().vertex);

    // a.borrow_mut().vertex.position[0] = 5.0;

    println!("{:?}", (vertices[0].as_ref().borrow()).vertex);

    EMesh {
        vertices,
        edges,
        // loops,
        // faces,
    }
}

// pub fn edges_of_face(emesh: EMesh, face_pos: usize) -> Vec<*const EEdge> {
//     let face = &emesh.faces[0];
//     let mut edges: Vec<*const EEdge> = vec![];

//     let mut to_get = face.loop_start;

//     for i in 0..face.loop_len {
//         unsafe {
//             edges.push((*to_get).edge);
//             to_get = (*to_get).next.unwrap();
//         }
//     }

//     edges
// }
