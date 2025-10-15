use crate::data::vertex::Vertex;

use super::{
    bm_edge::bm_edge_kill,
    bmesh::{BMesh, EdgeKey},
};

#[derive(Debug)]
pub struct BMVert {
    pub edge: Option<EdgeKey>,
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

pub fn bm_vert_create(bmesh: &mut BMesh) -> super::bmesh::VertKey {
    bmesh.vertices.insert(BMVert::from((0.0, 0.0)))
}

#[allow(dead_code)]
pub fn bm_vert_kill(bmesh: &mut BMesh, vert: super::bmesh::VertKey) {
    while let Some(edge) = bmesh.vertices.get(vert).and_then(|v| v.edge) {
        bm_edge_kill(bmesh, edge);
    }

    bm_kill_only_vert(bmesh, vert);
}

pub fn bm_kill_only_vert(bmesh: &mut BMesh, vert: super::bmesh::VertKey) {
    bmesh.vertices.remove(vert);
}
