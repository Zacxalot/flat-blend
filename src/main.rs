#![allow(dead_code)]
mod data;
mod opengl;
mod shapes;
// mod vulkan;

use data::mesh::bmesh::bm_triangulate;
use opengl::{flat_blend_state::FlatBlendState, structs::Object};
use shapes::square::create_square;

fn main() {
    let mut square = create_square();
    let (vertices, indices) = bm_triangulate(&mut square);
    let my_object = Object { vertices, indices };

    miniquad::start(miniquad::conf::Conf::default(), |ctx| {
        Box::new(FlatBlendState::new(ctx, vec![my_object]))
    });

    // run_event_loop(event_loop);
}
