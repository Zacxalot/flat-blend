use std::mem::ManuallyDrop;

use crate::data::vertex::Vertex;

use super::bm_edge::BMEdge;
#[derive(Debug)]
pub struct BMVert {
    pub edge: Option<*mut ManuallyDrop<BMEdge>>,
    pub vertex: Vertex,
}

impl From<(f32, f32)> for BMVert {
    fn from(input: (f32, f32)) -> Self {
        BMVert {
            edge: None,
            vertex: Vertex::from(input),
        }
    }
}
