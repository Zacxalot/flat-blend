use std::sync::Arc;

use vulkano::{
    buffer::{CpuAccessibleBuffer, CpuBufferPool, TypedBufferAccess},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder,
        CommandBufferExecFuture, CommandBufferUsage, RenderPassBeginInfo, SubpassContents,
    },
    descriptor_set::{
        allocator::StandardDescriptorSetAllocator, PersistentDescriptorSet, WriteDescriptorSet,
    },
    device::{Device, Queue},
    pipeline::{
        graphics::{
            input_assembly::InputAssemblyState,
            rasterization::{CullMode, PolygonMode, RasterizationState},
            vertex_input::BuffersDefinition,
            viewport::{Viewport, ViewportState},
        },
        GraphicsPipeline, Pipeline, PipelineBindPoint, StateMode,
    },
    render_pass::{Framebuffer, RenderPass, Subpass},
    shader::ShaderModule,
    swapchain::{
        acquire_next_image, AcquireError, PresentFuture, Surface, Swapchain,
        SwapchainAcquireFuture, SwapchainCreateInfo, SwapchainCreationError, SwapchainPresentInfo,
    },
    sync::{FenceSignalFuture, GpuFuture, JoinFuture},
};
use winit::window::Window;

use crate::{
    data::vertex::Vertex,
    shaders::flat::{self, vs::ty::Data},
};

use super::{
    swapchain::{self, window_size_dependent_setup},
    view::get_ortho,
};

pub fn create_pipeline(
    render_pass: Arc<RenderPass>,
    vs: Arc<ShaderModule>,
    fs: Arc<ShaderModule>,
    device: Arc<Device>,
    polygon_mode: PolygonMode,
) -> Arc<GraphicsPipeline> {
    let rasterization_state = RasterizationState {
        cull_mode: StateMode::Fixed(CullMode::None),
        polygon_mode,
        ..Default::default()
    };

    GraphicsPipeline::start()
        .render_pass(Subpass::from(render_pass, 0).unwrap())
        // We need to indicate the layout of the vertices.
        .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
        // The content of the vertex buffer describes a list of triangles.
        .input_assembly_state(InputAssemblyState::new())
        // A Vulkan shader can in theory contain multiple entry points, so we have to specify
        // which one.
        .vertex_shader(vs.entry_point("main").unwrap(), ())
        // Use a resizable viewport set to draw over the entire window
        .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
        .rasterization_state(rasterization_state)
        // See `vertex_shader`.
        .fragment_shader(fs.entry_point("main").unwrap(), ())
        // Now that our builder is filled, we call `build()` to obtain an actual pipeline.
        .build(device)
        .unwrap()
}

pub fn update_pipeline(
    recreate_swapchain: &mut bool,
    surface: Arc<Surface>,
    swapchain: &mut Arc<Swapchain>,
    previous_frame_end: &mut Option<Box<dyn GpuFuture>>,
    framebuffers: &mut Vec<Arc<Framebuffer>>,
    render_pass: Arc<RenderPass>,
    viewport: &mut Viewport,
    uniform_buffer: CpuBufferPool<Data>,
    pipeline: Arc<GraphicsPipeline>,
    descriptor_set_allocator: &StandardDescriptorSetAllocator,
    command_buffer_allocator: &StandardCommandBufferAllocator,
    queue: Arc<Queue>,
    vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
    index_buffer: Arc<CpuAccessibleBuffer<[u32]>>,
) -> Option<
    Result<
        FenceSignalFuture<
            PresentFuture<
                CommandBufferExecFuture<JoinFuture<Box<dyn GpuFuture>, SwapchainAcquireFuture>>,
            >,
        >,
        vulkano::sync::FlushError,
    >,
> {
    let window = surface.object().unwrap().downcast_ref::<Window>().unwrap();
    let dimensions = window.inner_size();
    if dimensions.width == 0 || dimensions.height == 0 {
        return None;
    }

    previous_frame_end.as_mut().unwrap().cleanup_finished();

    if *recreate_swapchain {
        println!("Recreate swapchain");
        let (new_swapchain, new_images) = match swapchain.recreate(SwapchainCreateInfo {
            image_extent: dimensions.into(),
            ..swapchain.create_info()
        }) {
            Ok(r) => r,
            Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return None,
            Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
        };

        *(swapchain) = new_swapchain;
        *(framebuffers) = window_size_dependent_setup(&new_images, render_pass.clone(), viewport);
        *(recreate_swapchain) = false;
    }

    let uniform_buffer_subbuffer = {
        let uniform_data = flat::vs::ty::Data {
            view: get_ortho(swapchain.clone()).into(),
        };

        uniform_buffer.from_data(uniform_data).unwrap()
    };

    let layout = pipeline.layout().set_layouts().get(0).unwrap();
    let set = PersistentDescriptorSet::new(
        descriptor_set_allocator,
        layout.clone(),
        [WriteDescriptorSet::buffer(0, uniform_buffer_subbuffer)],
    )
    .unwrap();

    let (image_index, suboptimal, acquire_future) =
        match acquire_next_image(swapchain.clone(), None) {
            Ok(r) => r,
            Err(AcquireError::OutOfDate) => {
                *(recreate_swapchain) = true;
                return None;
            }
            Err(e) => panic!("Failed to acquire next image: {:?}", e),
        };

    if suboptimal {
        *(recreate_swapchain) = true;
    }

    let mut builder = AutoCommandBufferBuilder::primary(
        command_buffer_allocator,
        queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    builder
        .begin_render_pass(
            RenderPassBeginInfo {
                clear_values: vec![Some([0.02, 0.02, 0.02, 1.0].into())],
                ..RenderPassBeginInfo::framebuffer(framebuffers[image_index as usize].clone())
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

    Some(
        previous_frame_end
            .take()
            .unwrap()
            .join(acquire_future)
            .then_execute(queue.clone(), command_buffer)
            .unwrap()
            .then_swapchain_present(
                queue.clone(),
                SwapchainPresentInfo::swapchain_image_index(swapchain.clone(), image_index),
            )
            .then_signal_fence_and_flush(),
    )
}
