mod data;
mod shaders;
mod shapes;
mod vulkan;

use std::{rc::Rc, sync::Arc};

use bytemuck::{Pod, Zeroable};

use data::edit_mesh::gen_square;
use lyon::{
    geom::point,
    lyon_tessellation::{
        geometry_builder::simple_builder, FillOptions, FillTessellator, VertexBuffers,
    },
    math::Point,
    path::Path,
};
use shapes::circle::create_circle;
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
    pipeline::{graphics::viewport::Viewport, Pipeline, PipelineBindPoint},
    swapchain::{
        acquire_next_image, AcquireError, SwapchainCreateInfo, SwapchainCreationError,
        SwapchainPresentInfo,
    },
    sync::{self, FlushError, GpuFuture},
    VulkanLibrary,
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{
    data::vertex::Vertex,
    shaders::flat,
    vulkan::{
        device::get_device,
        pipeline::create_pipeline,
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

fn tesselate_path(path: &Path) -> VertexBuffers<Point, u16> {
    let mut buffers: VertexBuffers<Point, u16> = VertexBuffers::new();

    {
        let mut vertex_builder = simple_builder(&mut buffers);

        // Create the tessellator.
        let mut tessellator = FillTessellator::new();

        // Compute the tessellation.
        tessellator
            .tessellate_path(path, &FillOptions::default(), &mut vertex_builder)
            .unwrap();
    }

    buffers
}

fn vulkano_init(vertices: Vec<Vertex>, indices: Vec<u16>) {
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

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
    struct Vertex {
        position: [f32; 2],
    }
    impl_vertex!(Vertex, position);

    let vertex_buffer = CpuAccessibleBuffer::from_iter(
        &memory_allocator,
        BufferUsage {
            vertex_buffer: true,
            ..BufferUsage::empty()
        },
        false,
        vertices,
    )
    .unwrap();

    let index_buffer = CpuAccessibleBuffer::from_iter(
        &memory_allocator,
        BufferUsage {
            index_buffer: true,
            ..BufferUsage::empty()
        },
        false,
        indices,
    )
    .unwrap();

    let uniform_buffer = CpuBufferPool::<flat::vs::ty::Data>::new(
        memory_allocator,
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

    let pipeline = create_pipeline(render_pass.clone(), vs, fs, device.clone());

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
        Event::RedrawEventsCleared => {
            let window = surface.object().unwrap().downcast_ref::<Window>().unwrap();
            let dimensions = window.inner_size();
            if dimensions.width == 0 || dimensions.height == 0 {
                return;
            }

            previous_frame_end.as_mut().unwrap().cleanup_finished();

            if recreate_swapchain {
                let (new_swapchain, new_images) = match swapchain.recreate(SwapchainCreateInfo {
                    image_extent: dimensions.into(),
                    ..swapchain.create_info()
                }) {
                    Ok(r) => r,
                    Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return,
                    Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
                };

                swapchain = new_swapchain;
                framebuffers =
                    window_size_dependent_setup(&new_images, render_pass.clone(), &mut viewport);
                recreate_swapchain = false;
            }

            let uniform_buffer_subbuffer = {
                let uniform_data = flat::vs::ty::Data {
                    view: get_ortho(swapchain.clone()).into(),
                };

                uniform_buffer.from_data(uniform_data).unwrap()
            };

            let layout = pipeline.layout().set_layouts().get(0).unwrap();
            let set = PersistentDescriptorSet::new(
                &descriptor_set_allocator,
                layout.clone(),
                [WriteDescriptorSet::buffer(0, uniform_buffer_subbuffer)],
            )
            .unwrap();

            let (image_index, suboptimal, acquire_future) =
                match acquire_next_image(swapchain.clone(), None) {
                    Ok(r) => r,
                    Err(AcquireError::OutOfDate) => {
                        recreate_swapchain = true;
                        return;
                    }
                    Err(e) => panic!("Failed to acquire next image: {:?}", e),
                };

            if suboptimal {
                recreate_swapchain = true;
            }

            let mut builder = AutoCommandBufferBuilder::primary(
                &command_buffer_allocator,
                queue.queue_family_index(),
                CommandBufferUsage::OneTimeSubmit,
            )
            .unwrap();

            builder
                .begin_render_pass(
                    RenderPassBeginInfo {
                        clear_values: vec![Some([0.0, 0.0, 1.0, 1.0].into())],
                        ..RenderPassBeginInfo::framebuffer(
                            framebuffers[image_index as usize].clone(),
                        )
                    },
                    SubpassContents::Inline,
                )
                .unwrap()
                .set_viewport(0, [viewport.clone()])
                .bind_pipeline_graphics(pipeline.clone())
                .bind_descriptor_sets(
                    PipelineBindPoint::Graphics,
                    pipeline.layout().clone(),
                    0,
                    set,
                )
                .bind_vertex_buffers(0, vertex_buffer.clone())
                .bind_index_buffer(index_buffer.clone())
                .draw_indexed(index_buffer.len() as u32, 1, 0, 0, 0)
                .unwrap()
                .end_render_pass()
                .unwrap();

            let command_buffer = builder.build().unwrap();

            let future = previous_frame_end
                .take()
                .unwrap()
                .join(acquire_future)
                .then_execute(queue.clone(), command_buffer)
                .unwrap()
                .then_swapchain_present(
                    queue.clone(),
                    SwapchainPresentInfo::swapchain_image_index(swapchain.clone(), image_index),
                )
                .then_signal_fence_and_flush();

            match future {
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
        _ => (),
    });
}

fn main() {
    // let buffers = create_circle();
    // // let buffers = tesselate_path(&path);

    // let vertices = buffers
    //     .vertices
    //     .iter()
    //     .map(|vertex| Vertex::from(*vertex))
    //     .collect::<Vec<Vertex>>();

    // let indices = buffers.indices.to_vec();

    // vulkano_init(vertices, indices);

    let mut EMesh = gen_square();

    EMesh.vertices.clear();
    EMesh.edges.clear();
    println!("{:?}", Rc::strong_count(&(*EMesh.edges[0]).borrow().v0));

    // let edges = edges_of_face(EMesh, 0);

    // for edge in edges {
    //     unsafe { println!("{}, {}", (*edge).0, (*edge).1) }
    // }
}
