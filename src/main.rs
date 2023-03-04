mod data;
mod shapes;
mod vulkan;

use std::sync::Arc;

use data::vertex::Vertex;
use enum_map::enum_map;
use shapes::square::create_square;
use vulkan::{
    buffers::{IndexBuffers, VertexBuffers},
    init::{vulkano_init, VulkanState},
    pipeline::render_frame,
};
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer, CpuBufferPool},
    command_buffer::allocator::StandardCommandBufferAllocator,
    descriptor_set::allocator::StandardDescriptorSetAllocator,
    device::{Device, Queue},
    instance::{Instance, InstanceCreateInfo},
    memory::allocator::{MemoryUsage, StandardMemoryAllocator},
    pipeline::graphics::{rasterization::PolygonMode, viewport::Viewport},
    render_pass::Framebuffer,
    swapchain::Surface,
    sync::{self, FlushError, GpuFuture},
    VulkanLibrary,
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{self, ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{
    data::mesh::bmesh::bm_edge_list,
    vulkan::{
        device::get_device,
        render_passes::solid::{solid_draw_pass, solid_draw_pipeline},
        shaders::shader_loader::load_shaders,
        swapchain::create_swapchain,
    },
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

            let rel_x = 6.0 * ((position.x / width) * 2.0 - 1.0);
            let rel_y = 6.0 * (1.0 - (position.y / height) * 2.0);

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
