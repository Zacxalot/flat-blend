mod data;
mod shaders;
mod shapes;
mod vulkan;

use std::sync::Arc;

use bytemuck::{Pod, Zeroable};

use data::mesh::bmesh::bm_triangulate;
use lyon::{geom::point, path::Path};

use shapes::square::create_square;
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer, CpuBufferPool, TypedBufferAccess},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
        RenderPassBeginInfo, SubpassContents,
    },
    descriptor_set::{
        allocator::StandardDescriptorSetAllocator, PersistentDescriptorSet, WriteDescriptorSet,
    },
    impl_vertex,
    instance::{Instance, InstanceCreateInfo},
    memory::allocator::{MemoryUsage, StandardMemoryAllocator},
    pipeline::{
        graphics::{rasterization::PolygonMode, viewport::Viewport},
        Pipeline, PipelineBindPoint,
    },
    swapchain::{
        acquire_next_image, AcquireError, SwapchainCreateInfo, SwapchainCreationError,
        SwapchainPresentInfo,
    },
    sync::{self, FlushError, GpuFuture},
    VulkanLibrary,
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{
    data::mesh::bmesh::bm_edge_list,
    shaders::flat,
    vulkan::{
        device::get_device,
        pipeline::{create_pipeline, update_pipeline},
        swapchain::{create_swapchain, window_size_dependent_setup},
        view::get_ortho,
    },
};

fn build_path() -> Path {
    let mut path_builder = Path::builder();
    path_builder.begin(point(-1.0, 0.0));
    path_builder.line_to(point(0.0, 1.0));
    path_builder.line_to(point(1.0, 0.0));
    path_builder.line_to(point(0.0, -1.0));
    path_builder.end(true);
    path_builder.build()
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
    let mut surface = WindowBuilder::new()
        .build_vk_surface(&event_loop, instance.clone())
        .unwrap();

    let (device, mut queues) = get_device(instance, surface.clone());

    let queue = queues.next().unwrap();

    let (mut swapchain, images) = create_swapchain(device.clone(), surface.clone());

    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
    struct Vertex {
        position: [f32; 2],
    }
    impl_vertex!(Vertex, position);

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
    //     [],
    // )
    // .unwrap();

    let uniform_buffer = CpuBufferPool::<flat::vs::ty::Data>::new(
        memory_allocator.clone(),
        BufferUsage {
            uniform_buffer: true,
            ..BufferUsage::empty()
        },
        MemoryUsage::Upload,
    );

    let vs = flat::vs::load(device.clone()).unwrap();
    let fs = flat::fs::load(device.clone()).unwrap();

    let render_pass = vulkano::single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: swapchain.image_format(),
                samples: 1,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {}
        }
    )
    .unwrap();

    let mut pipeline = create_pipeline(
        render_pass.clone(),
        vs.clone(),
        fs.clone(),
        device.clone(),
        PolygonMode::Fill,
    );

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
                if let Some(virtual_keykode) = input.virtual_keycode {
                    match virtual_keykode {
                        VirtualKeyCode::Z => {
                            let mut polygon_mode = PolygonMode::Fill;
                            if pipeline.rasterization_state().polygon_mode == PolygonMode::Fill {
                                polygon_mode = PolygonMode::Line;
                            }

                            pipeline = create_pipeline(
                                render_pass.clone(),
                                vs.clone(),
                                fs.clone(),
                                device.clone(),
                                polygon_mode,
                            );
                            if let Some(result) = update_pipeline(
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
                                        previous_frame_end =
                                            Some(sync::now(device.clone()).boxed());
                                    }
                                    Err(e) => {
                                        panic!("Failed to flush future: {:?}", e);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
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
            if let Some(result) = update_pipeline(
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
