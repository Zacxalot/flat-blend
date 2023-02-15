use slab::Slab;

use super::{bm_edge::BMEdge, bm_face::BMFace, bm_loop::BMLoop, bm_vert::BMVert};

pub struct BMesh {
    pub vertices: Slab<BMVert>,
    pub edges: Slab<BMEdge>,
    pub loops: Slab<BMLoop>,
    pub faces: Slab<BMFace>,
}

impl BMesh {
    pub fn new() -> BMesh {
        let vertices = Slab::new();
        let edges = Slab::new();
        let loops = Slab::new();
        let faces = Slab::new();

        BMesh {
            vertices,
            edges,
            loops,
            faces,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{
        mesh::{
            bm_edge::bm_edge_create,
            bm_vert::{bm_vert_create, bm_vert_kill},
        },
        vertex::Vertex,
    };

    use super::BMesh;

    #[test]
    fn create_edge_remove_vert() {
        let mut bmesh = BMesh::new();

        unsafe {
            let mut v0 = bm_vert_create(&mut bmesh);
            (*v0).vertex = Vertex::from((-1.0, 0.0));
            let mut v1 = bm_vert_create(&mut bmesh);
            (*v1).vertex = Vertex::from((1.0, 0.0));

            let e0 = bm_edge_create(&mut bmesh, v0, v1);

            assert_eq!((*e0).v0, v0);
            assert_eq!((*e0).v1, v1);
            assert_eq!((*v0).edge, (*v1).edge);
            assert_eq!(bmesh.vertices.len(), 2);
            assert_eq!(bmesh.edges.len(), 1);

            bm_vert_kill(&mut bmesh, v0);

            assert_eq!((*v1).edge, None);
            assert_eq!(bmesh.vertices.len(), 1);
            assert_eq!(bmesh.edges.len(), 0);
        }
    }
}
