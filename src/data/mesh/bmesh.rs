use slotmap::{new_key_type, SlotMap};

use crate::data::vertex::{Index, Vertex};

use super::{
    bm_edge::BMEdge,
    bm_face::BMFace,
    bm_loop::{BMLoop, BMLoopIterator},
    bm_vert::BMVert,
};

new_key_type! {
    pub struct VertKey;
    pub struct EdgeKey;
    pub struct LoopKey;
    pub struct FaceKey;
}

pub struct BMesh {
    pub vertices: SlotMap<VertKey, BMVert>,
    pub edges: SlotMap<EdgeKey, BMEdge>,
    pub loops: SlotMap<LoopKey, BMLoop>,
    pub faces: SlotMap<FaceKey, BMFace>,
}

impl BMesh {
    pub fn new() -> BMesh {
        let vertices = SlotMap::with_key();
        let edges = SlotMap::with_key();
        let loops = SlotMap::with_key();
        let faces = SlotMap::with_key();

        BMesh {
            vertices,
            edges,
            loops,
            faces,
        }
    }
}

#[allow(dead_code)]
pub fn bm_triangulate(bmesh: &BMesh) -> (Vec<Vertex>, Vec<Index>) {
    let mut all_bm_vertices: Vec<VertKey> = vec![];
    let mut all_indices: Vec<Index> = vec![];

    for (_, face) in &bmesh.faces {
        if let Some(loop_start) = face.loop_start {
            let vertices = BMLoopIterator::new(bmesh, loop_start)
                .map(|l| bmesh.loops[l].vertex)
                .collect::<Vec<VertKey>>();

            let flattened_verts = vertices
                .iter()
                .flat_map(|v| [bmesh.vertices[*v].vertex.pos.x, bmesh.vertices[*v].vertex.pos.y])
                .collect::<Vec<f32>>();

            let indices = earcutr::earcut(&flattened_verts, &[], 2).unwrap();

            for index in indices {
                if let Some(position) = all_bm_vertices
                    .iter()
                    .position(|val| val == &vertices[index])
                {
                    all_indices.push(position as u32);
                } else {
                    all_bm_vertices.push(vertices[index]);
                    all_indices.push((all_bm_vertices.len() - 1) as Index);
                }
            }
        }
    }

    let all_vertices = all_bm_vertices
        .iter()
        .map(|v| bmesh.vertices[*v].vertex)
        .collect::<Vec<Vertex>>();
    (all_vertices, all_indices)
}

pub fn bm_edge_list(bmesh: &BMesh) -> Vec<Vertex> {
    bmesh
        .edges
        .iter()
        .flat_map(|(_, edge)| [bmesh.vertices[edge.v0].vertex, bmesh.vertices[edge.v1].vertex])
        .collect::<Vec<Vertex>>()
}

#[cfg(test)]
mod tests {
    use crate::data::{
        mesh::{
            bm_edge::bm_edge_create,
            bm_face::bm_face_create,
            bm_vert::{bm_vert_create, bm_vert_kill},
        },
        vertex::Vertex,
    };

    use super::BMesh;

    #[test]
    fn create_edge_remove_vert() {
        let mut bmesh = BMesh::new();

        let v0 = bm_vert_create(&mut bmesh);
        bmesh.vertices[v0].vertex = Vertex::from((-1.0, 0.0));
        let v1 = bm_vert_create(&mut bmesh);
        bmesh.vertices[v1].vertex = Vertex::from((1.0, 0.0));

        let e0 = bm_edge_create(&mut bmesh, v0, v1);

        assert_eq!(bmesh.edges[e0].v0, v0);
        assert_eq!(bmesh.edges[e0].v1, v1);
        assert_eq!(bmesh.vertices[v0].edge, bmesh.vertices[v1].edge);
        assert_eq!(bmesh.vertices.len(), 2);
        assert_eq!(bmesh.edges.len(), 1);

        bm_vert_kill(&mut bmesh, v0);

        assert_eq!(bmesh.vertices[v1].edge, None);
        assert_eq!(bmesh.vertices.len(), 1);
        assert_eq!(bmesh.edges.len(), 0);
    }

    #[test]
    fn create_2_edges_remove_vert() {
        let mut bmesh = BMesh::new();

        let v0 = bm_vert_create(&mut bmesh);
        bmesh.vertices[v0].vertex = Vertex::from((-1.0, -1.0));
        let v1 = bm_vert_create(&mut bmesh);
        bmesh.vertices[v1].vertex = Vertex::from((1.0, -1.0));
        let v2 = bm_vert_create(&mut bmesh);
        bmesh.vertices[v2].vertex = Vertex::from((1.0, 1.0));

        let e0 = bm_edge_create(&mut bmesh, v0, v1);
        let e1 = bm_edge_create(&mut bmesh, v1, v2);

        assert_eq!(bmesh.edges[e0].v0, v0);
        assert_eq!(bmesh.edges[e0].v1, v1);
        assert_eq!(bmesh.edges[e1].v0, v1);
        assert_eq!(bmesh.edges[e1].v1, v2);

        assert_eq!(bmesh.vertices.len(), 3);
        assert_eq!(bmesh.edges.len(), 2);

        bm_vert_kill(&mut bmesh, v1);

        assert_eq!(bmesh.vertices.len(), 2);
        assert_eq!(bmesh.edges.len(), 0);
        assert_eq!(bmesh.vertices[v0].edge, None);
        assert_eq!(bmesh.vertices[v2].edge, None);
    }

    #[test]
    fn create_face() {
        let mut bmesh = BMesh::new();

        let v0 = bm_vert_create(&mut bmesh);
        bmesh.vertices[v0].vertex = Vertex::from((-1.0, -1.0));
        let v1 = bm_vert_create(&mut bmesh);
        bmesh.vertices[v1].vertex = Vertex::from((1.0, -1.0));
        let v2 = bm_vert_create(&mut bmesh);
        bmesh.vertices[v2].vertex = Vertex::from((1.0, 1.0));

        let e0 = bm_edge_create(&mut bmesh, v0, v1);
        let e1 = bm_edge_create(&mut bmesh, v1, v2);
        let e2 = bm_edge_create(&mut bmesh, v2, v0);

        let _f0 = bm_face_create(&mut bmesh, &[v0, v1, v2], &[e0, e1, e2]);

        assert_eq!(bmesh.edges[e0].v0, v0);
        assert_eq!(bmesh.edges[e0].v1, v1);
        assert_eq!(bmesh.edges[e1].v0, v1);
        assert_eq!(bmesh.edges[e1].v1, v2);
        assert_eq!(bmesh.edges[e2].v0, v2);
        assert_eq!(bmesh.edges[e2].v1, v0);

        assert_eq!(bmesh.vertices.len(), 3);
        assert_eq!(bmesh.edges.len(), 3);
        assert_eq!(bmesh.faces.len(), 1);
        assert_eq!(bmesh.loops.len(), 3);

        bm_vert_kill(&mut bmesh, v1);

        assert_eq!(bmesh.vertices.len(), 2);
        assert_eq!(bmesh.edges.len(), 1);
        assert_eq!(bmesh.faces.len(), 0);
        assert_eq!(bmesh.loops.len(), 0);
    }

    #[test]
    fn create_square() {
        let mut bmesh = BMesh::new();

        let v0 = bm_vert_create(&mut bmesh);
        bmesh.vertices[v0].vertex = Vertex::from((-1.0, -1.0));
        let v1 = bm_vert_create(&mut bmesh);
        bmesh.vertices[v1].vertex = Vertex::from((1.0, -1.0));
        let v2 = bm_vert_create(&mut bmesh);
        bmesh.vertices[v2].vertex = Vertex::from((1.0, 1.0));
        let v3 = bm_vert_create(&mut bmesh);
        bmesh.vertices[v3].vertex = Vertex::from((1.0, 1.0));

        let e0 = bm_edge_create(&mut bmesh, v0, v1);
        let e1 = bm_edge_create(&mut bmesh, v1, v2);
        let e2 = bm_edge_create(&mut bmesh, v2, v3);
        let e3 = bm_edge_create(&mut bmesh, v3, v0);

        let _f0 = bm_face_create(&mut bmesh, &[v0, v1, v2, v3], &[e0, e1, e2, e3]);

        assert_eq!(bmesh.vertices.len(), 4);
        assert_eq!(bmesh.edges.len(), 4);
        assert_eq!(bmesh.faces.len(), 1);
        assert_eq!(bmesh.loops.len(), 4);

        bm_vert_kill(&mut bmesh, v1);

        assert_eq!(bmesh.vertices.len(), 3);
        assert_eq!(bmesh.edges.len(), 2);
        assert_eq!(bmesh.faces.len(), 0);
        assert_eq!(bmesh.loops.len(), 0);
    }
}
