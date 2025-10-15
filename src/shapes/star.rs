use crate::data::{
    mesh::{
        bm_edge::bm_edge_create, bm_face::bm_face_create, bm_vert::bm_vert_create, bmesh::BMesh,
    },
    vertex::Vertex,
};

#[allow(dead_code)]
pub fn create_star() -> BMesh {
    let mut bmesh = BMesh::new();

    // Create a 5-pointed star with 10 vertices (5 outer points, 5 inner points)
    let outer_radius = 1.0;
    let inner_radius = 0.4;
    let num_points = 5;

    // Create vertices alternating between outer and inner points
    let mut vertices = Vec::new();

    for i in 0..num_points {
        // Outer point
        let angle_outer = (i as f32) * 2.0 * std::f32::consts::PI / (num_points as f32);
        let v_outer = bm_vert_create(&mut bmesh);
        bmesh.vertices[v_outer].vertex = Vertex::from((
            outer_radius * angle_outer.cos(),
            outer_radius * angle_outer.sin(),
        ));
        vertices.push(v_outer);

        // Inner point (offset by half the angle between outer points)
        let angle_inner = angle_outer + std::f32::consts::PI / (num_points as f32);
        let v_inner = bm_vert_create(&mut bmesh);
        bmesh.vertices[v_inner].vertex = Vertex::from((
            inner_radius * angle_inner.cos(),
            inner_radius * angle_inner.sin(),
        ));
        vertices.push(v_inner);
    }

    // Create edges connecting the vertices in order
    let mut edges = Vec::new();
    for i in 0..vertices.len() {
        let v0 = vertices[i];
        let v1 = vertices[(i + 1) % vertices.len()];
        let edge = bm_edge_create(&mut bmesh, v0, v1);
        edges.push(edge);
    }

    // Create the face
    bm_face_create(&mut bmesh, &vertices, &edges);

    bmesh
}
