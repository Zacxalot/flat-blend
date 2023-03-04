#![allow(dead_code)]
mod data;
mod shapes;
mod vulkan;

use vulkan::{
    init::{vulkano_init, VulkanState},
    pipeline::render_frame,
};
use vulkano::sync::{self, FlushError, GpuFuture};

use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

fn main() {
    let (state, event_loop) = vulkano_init();
    run_event_loop(state, event_loop);
}

fn run_event_loop(mut state: VulkanState, event_loop: EventLoop<()>) {
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        }
        Event::WindowEvent {
            event: WindowEvent::Resized(_),
            ..
        } => {
            state.recreate_swapchain = true;
        }
        Event::WindowEvent {
            event: WindowEvent::MouseInput { state, button, .. },
            ..
        } => match button {
            winit::event::MouseButton::Left => match state {
                winit::event::ElementState::Pressed => {}
                winit::event::ElementState::Released => {}
            },
            winit::event::MouseButton::Right => {}
            winit::event::MouseButton::Middle => {}
            winit::event::MouseButton::Other(_) => {}
        },
        Event::WindowEvent {
            event: WindowEvent::KeyboardInput { input, .. },
            ..
        } => {
            if input.state == ElementState::Pressed {
                #[allow(clippy::single_match)]
                match input.virtual_keycode {
                    Some(VirtualKeyCode::Z) => {
                        println!("Pressed Z");
                        // let mut polygon_mode = PolygonMode::Fill;
                        // if pipeline.rasterization_state().polygon_mode == PolygonMode::Fill {
                        //     polygon_mode = PolygonMode::Line;
                        // }
                    }
                    _ => {}
                }
            }
        }
        Event::WindowEvent {
            event: WindowEvent::CursorMoved { position, .. },
            ..
        } => {
            let window = state
                .surface
                .object()
                .unwrap()
                .downcast_ref::<Window>()
                .unwrap();
            let width = window.inner_size().width as f64;
            let height = window.inner_size().height as f64;

            let _rel_x = 6.0 * ((position.x / width) * 2.0 - 1.0);
            let _rel_y = 6.0 * (1.0 - (position.y / height) * 2.0);

            // square_mesh
            //     .vertices
            //     .iter_mut()
            //     .next()
            //     .unwrap()
            //     .1
            //     .vertex
            //     .position = [rel_x as f32, rel_y as f32];

            // let (vertices, indices) = bm_triangulate(&mut square_mesh);
            // let vertices = bm_edge_list(&mut square_mesh);

            // vertex_buffer = CpuAccessibleBuffer::from_iter(
            //     &memory_allocator,
            //     BufferUsage {
            //         vertex_buffer: true,
            //         ..BufferUsage::empty()
            //     },
            //     false,
            //     vertices,
            // )
            // .unwrap();

            // index_buffer = CpuAccessibleBuffer::from_iter(
            //     &memory_allocator,
            //     BufferUsage {
            //         index_buffer: true,
            //         ..BufferUsage::empty()
            //     },
            //     false,
            //     indices,
            // )
            // .unwrap();
        }
        Event::RedrawEventsCleared => {
            if let Some(result) = render_frame(&mut state) {
                match result {
                    Ok(future) => {
                        state.previous_frame_end = Some(future.boxed());
                    }
                    Err(FlushError::OutOfDate) => {
                        state.recreate_swapchain = true;
                        state.previous_frame_end = Some(sync::now(state.device.clone()).boxed());
                    }
                    Err(e) => {
                        panic!("Failed to flush future: {:?}", e);
                    }
                }
            }
        }
        _ => (),
    });
}
