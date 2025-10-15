#![allow(dead_code)]
mod data;
mod input;
mod opengl;
mod shapes;
mod ui;

use std::{cell::RefCell, rc::Rc};

use glam::Vec2;
use opengl::{
    flat_blend_state::FlatBlendState,
    structs::{Mesh, Object},
};
use rand::Rng;
use shapes::square::create_square;

use crate::opengl::structs::{Colour, Material};

fn main() {
    let square = create_square();
    let star = shapes::star::create_star();

    let (mesh, _verts, _indices) = Mesh::new(star, 0);

    let mut objects: Vec<Object> = vec![];
    let mut rng = rand::rng();

    for y in 0..500 {
        for x in 0..500 {
            objects.push(Object::new(
                mesh.clone(),
                Vec2::new(x as f32 * 250.0, y as f32 * 250.0),
                std::f32::consts::PI * rng.random::<f32>() * 2.0,
                Vec2::new(100.0, 100.0),
                Rc::new(RefCell::new(Material {
                    colour: Colour::new(rng.random(), rng.random(), rng.random(), 1.0),
                })),
            ))
        }
    }

    println!("objects: {}", objects.len());

    miniquad::start(
        miniquad::conf::Conf::default(),
        |ctx: &mut miniquad::Context| Box::new(FlatBlendState::new(ctx, objects, vec![mesh])),
    );

    // run_event_loop(event_loop);
}
