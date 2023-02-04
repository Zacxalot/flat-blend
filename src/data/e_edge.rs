use std::{cell::RefCell, rc::Rc};

use super::e_vert::{EVert, EVertRc};

pub type EEdgeRc = Rc<RefCell<EEdge>>;

#[derive(Debug)]
pub struct EEdge {
    pub v0: EVertRc,
    pub v1: EVertRc,
}

// impl EEdge {
//     fn new(verts: (EVertRc, EVertRc)) -> Self {
//         EEdge {
//             v0: verts.0,
//             v1: verts.1,
//         }
//     }
// }
