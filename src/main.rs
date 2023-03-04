mod data;
mod shapes;
mod vulkan;

use std::sync::Arc;

use shapes::square::create_square;
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer, CpuBufferPool},
    command_buffer::allocator::StandardCommandBufferAllocator,
    descriptor_set::allocator::StandardDescriptorSetAllocator,
    device::Device,
    instance::{Instance, InstanceCreateInfo},
    memory::allocator::{MemoryUsage, StandardMemoryAllocator},
    pipeline::graphics::{rasterization::PolygonMode, viewport::Viewport},
    swapchain::Surface,
    sync::{self, FlushError, GpuFuture},
    VulkanLibrary,
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{
    data::mesh::bmesh::bm_edge_list,
    vulkan::{
        device::get_device,
        pipeline::render_frame,
        render_passes::solid::{solid_draw_pass, solid_draw_pipeline},
        shaders::shader_loader::load_shaders,
        swapchain::{create_swapchain, window_size_dependent_setup},
    },
};

struct VulkanState {
    device: Arc<Device>,
    surface: Arc<Surface>,
}

fn vulkano_init() {
    let library = VulkanLibrary::new().unwrap();
    let required_extensions = vulkano_win::required_extensions(&library);

    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            enabled_extensions: required_extensions,
            enumerate_portability: true,
            ..Default::default()
        },
    )
    .unwrap();

    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .build_vk_surface(&event_loop, instance.clone())
        .unwrap();

    let (device, mut queues) = get_device(instance, surface.clone());

    let queue = queues.next().unwrap();

    let (mut swapchain, images) = create_swapchain(device.clone(), surface.clone());

    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    let mut square_mesh = create_square();
    // let (vertices, indices) = bm_triangulate(&mut square_mesh);
    let vertices = bm_edge_list(&mut square_mesh);
    println!("{:?}", vertices);

    let mut vertex_buffer = CpuAccessibleBuffer::from_iter(
        &memory_allocator,
        BufferUsage {
            vertex_buffer: true,
            ..BufferUsage::empty()
        },
        false,
        vertices,
    )
    .unwrap();

    // let mut index_buffer = CpuAccessibleBuffer::from_iter(
    //     &memory_allocator,
    //     BufferUsage {
    //         index_buffer: true,
    //         ..BufferUsage::empty()
    //     },
    //     false,
    //     [],#
    // )
    // .unwrap();

    let uniform_buffer = CpuBufferPool::<vulkan::shaders::flat::vs::ty::Data>::new(
        memory_allocator.clone(),
        BufferUsage {
            uniform_buffer: true,
            ..BufferUsage::empty()
        },
        MemoryUsage::Upload,
    );

    let shaders = load_shaders(device.clone());
    let render_pass = solid_draw_pass(device.clone(), swapchain.image_format()).unwrap();

    let pipeline = solid_draw_pipeline(render_pass.clone(), device.clone(), shaders).unwrap();

    let mut viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [0.0, 0.0],
        depth_range: 0.0..1.0,
    };

    let mut framebuffers = window_size_dependent_setup(&images, render_pass.clone(), &mut viewport);

    let descriptor_set_allocator = StandardDescriptorSetAllocator::new(device.clone());

    let command_buffer_allocator =
        StandardCommandBufferAllocator::new(device.clone(), Default::default());

    let mut recreate_swapchain = false;

    let mut previous_frame_end = Some(sync::now(device.clone()).boxed());

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
            recreate_swapchain = true;
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
                        let mut polygon_mode = PolygonMode::Fill;
                        if pipeline.rasterization_state().polygon_mode == PolygonMode::Fill {
                            polygon_mode = PolygonMode::Line;
                        }
                    }
                    _ => {}
                }
            }
        }
        Event::WindowEvent {
            event: WindowEvent::CursorMoved { position, .. },
            ..
        } => {
            let window = surface.object().unwrap().downcast_ref::<Window>().unwrap();
            let width = window.inner_size().width as f64;
            let height = window.inner_size().height as f64;

            let rel_x = 6.0 * ((position.x / width) * 2.0 - 1.0);
            let rel_y = 6.0 * (1.0 - (position.y / height) * 2.0);

            square_mesh
                .vertices
                .iter_mut()
                .next()
                .unwrap()
                .1
                .vertex
                .position = [rel_x as f32, rel_y as f32];

            // let (vertices, indices) = bm_triangulate(&mut square_mesh);
            let vertices = bm_edge_list(&mut square_mesh);

            vertex_buffer = CpuAccessibleBuffer::from_iter(
                &memory_allocator,
                BufferUsage {
                    vertex_buffer: true,
                    ..BufferUsage::empty()
                },
                false,
                vertices,
            )
            .unwrap();

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
            if let Some(result) = render_frame(
                &mut recreate_swapchain,
                surface.clone(),
                &mut swapchain,
                &mut previous_frame_end,
                &mut framebuffers,
                render_pass.clone(),
                &mut viewport,
                uniform_buffer.clone(),
                pipeline.clone(),
                &descriptor_set_allocator,
                &command_buffer_allocator,
                queue.clone(),
                vertex_buffer.clone(),
            ) {
                match result {
                    Ok(future) => {
                        previous_frame_end = Some(future.boxed());
                    }
                    Err(FlushError::OutOfDate) => {
                        recreate_swapchain = true;
                        previous_frame_end = Some(sync::now(device.clone()).boxed());
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

fn main() {
    vulkano_init();
}
