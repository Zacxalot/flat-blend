#![allow(dead_code)]
mod data;
mod input;
mod opengl;
mod shapes;
mod ui;

use glam::Vec2;
use opengl::{
    flat_blend_state::FlatBlendState,
    structs::{Mesh, Object},
};
use shapes::square::create_square;

fn main() {
    let square = create_square();

    let (mesh, _verts, _indices) = Mesh::new(square, 0);

    let mut objects: Vec<Object> = vec![];

    for y in 0..50 {
        for x in 0..50 {
            objects.push(Object {
                mesh: mesh.clone(),
                translation: Vec2::new(x as f32 * 2.5, y as f32 * 2.5),
            });
        }
    }

    println!("objects: {}", objects.len());

    miniquad::start(
        miniquad::conf::Conf::default(),
        |ctx: &mut miniquad::Context| Box::new(FlatBlendState::new(ctx, objects, vec![mesh])),
    );

    // run_event_loop(event_loop);
}
