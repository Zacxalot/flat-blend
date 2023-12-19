#![allow(dead_code)]
mod data;
mod input;
mod opengl;
mod shapes;

use data::mesh::bmesh::bm_triangulate;
use glam::Vec2;
use opengl::{flat_blend_state::FlatBlendState, structs::Object};
use shapes::square::create_square;

fn main() {
    let mut square = create_square();
    let (vertices, indices) = bm_triangulate(&mut square);
    let my_object = Object {
        vertices,
        indices,
        ..Default::default()
    };

    let (vertices, indices) = bm_triangulate(&mut square);
    let my_object2 = Object {
        vertices,
        indices,
        translation: Vec2::new(0.5, 0.5),
    };

    miniquad::start(miniquad::conf::Conf::default(), |ctx| {
        Box::new(FlatBlendState::new(ctx, vec![my_object, my_object2]))
    });

    // run_event_loop(event_loop);
}
