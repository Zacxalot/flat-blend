#![allow(dead_code)]
mod data;
mod input;
mod opengl;
mod shapes;
mod ui;

use std::{cell::RefCell, rc::Rc, sync::Arc};

use glam::Vec2;
use opengl::{
    flat_blend_state::FlatBlendState,
    structs::{Mesh, Object},
};
use shapes::square::create_square;

use crate::opengl::structs::{Colour, Material};

fn main() {
    let square = create_square();

    let (mesh, _verts, _indices) = Mesh::new(square, 0);

    let material = Rc::new(RefCell::new(Material {
        colour: Colour::new(1.0, 1.0, 0.0, 1.0),
    }));

    let mut objects: Vec<Object> = vec![];

    for y in 0..50 {
        for x in 0..50 {
            objects.push(Object {
                name: format!("Square {} {}", x, y),
                mesh: mesh.clone(),
                translation: Vec2::new(x as f32 * 2.5, y as f32 * 2.5),
                material: material.clone(),
            });
        }
    }

    println!("objects: {}", objects.len());

    miniquad::start(
        miniquad::conf::Conf::default(),
        |ctx: &mut miniquad::Context| {
            Box::new(FlatBlendState::new(ctx, Arc::new(objects), vec![mesh]))
        },
    );

    // run_event_loop(event_loop);
}
