use std::{cell::RefCell, rc::Rc};

use super::{e_edge::EEdgeRc, vertex::Vertex};

pub type EVertRc = Rc<RefCell<EVert>>;

#[derive(Debug)]
pub struct EVert {
    pub edge: Option<EEdgeRc>,
    pub vertex: Vertex,
}

impl From<(f32, f32)> for EVert {
    fn from(input: (f32, f32)) -> Self {
        EVert {
            edge: None,
            vertex: Vertex::from(input),
        }
    }
}
