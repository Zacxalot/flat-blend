use crate::data::{
    mesh::{
        bm_edge::bm_edge_create, bm_face::bm_face_create, bm_vert::bm_vert_create, bmesh::BMesh,
    },
    vertex::Vertex,
};

pub fn create_square() -> BMesh {
    let mut bmesh = BMesh::new();

    unsafe {
        let mut v0 = bm_vert_create(&mut bmesh);
        (*v0).vertex = Vertex::from((-1.0, -1.0));
        let mut v1 = bm_vert_create(&mut bmesh);
        (*v1).vertex = Vertex::from((1.0, -1.0));
        let mut v2 = bm_vert_create(&mut bmesh);
        (*v2).vertex = Vertex::from((1.0, 1.0));
        let mut v3 = bm_vert_create(&mut bmesh);
        (*v3).vertex = Vertex::from((-1.0, 1.0));

        let e0 = bm_edge_create(&mut bmesh, v0, v1);
        let e1 = bm_edge_create(&mut bmesh, v1, v2);
        let e2 = bm_edge_create(&mut bmesh, v2, v3);
        let e3 = bm_edge_create(&mut bmesh, v3, v0);

        bm_face_create(&mut bmesh, &[v0, v1, v2, v3], &[e0, e1, e2, e3]);
    }

    bmesh
}
