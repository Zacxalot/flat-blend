#![allow(dead_code)]
// mod data;
mod opengl;
// mod shapes;
// mod vulkan;

use opengl::flat_blend_state::FlatBlendState;

// use data::{mesh::bmesh::bm_triangulate, vertex::FBVertex};
// use shapes::square::create_square;
// use vulkan::init::{vulkano_init, VulkanState};
// use vulkano::{
//     buffer::{BufferUsage, CpuAccessibleBuffer},
//     sync::{self, FlushError, GpuFuture},
// };

// use vulkan::init::vulkano_init;
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

fn main() {
    // let (mut state, event_loop) = vulkano_init();

    // let mut square = create_square();
    // let (vertices, indices) = bm_triangulate(&mut square);

    let mut conf = miniquad::conf::Conf::default();

    miniquad::start(miniquad::conf::Conf::default(), |mut ctx| {
        Box::new(FlatBlendState::new(&mut ctx))
    });

    // run_event_loop(event_loop);
}

fn run_event_loop(event_loop: EventLoop<()>) {
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        }
        _ => (),
    });
}
