use std::mem::ManuallyDrop;

use super::{bm_edge::BMEdge, vertex::Vertex};
#[derive(Debug)]
pub struct BMVert {
    pub edge: Option<*mut ManuallyDrop<BMEdge>>,
    pub vertex: Vertex,
}

impl Drop for BMVert {
    fn drop(&mut self) {
        println!("DROPPING VERT");
    }
}

impl From<(f32, f32)> for BMVert {
    fn from(input: (f32, f32)) -> Self {
        BMVert {
            edge: None,
            vertex: Vertex::from(input),
        }
    }
}
