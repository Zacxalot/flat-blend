use std::sync::Arc;

use vulkano::{
    buffer::TypedBufferAccess,
    command_buffer::{
        AutoCommandBufferBuilder, PrimaryAutoCommandBuffer, RenderPassBeginInfo, SubpassContents,
    },
    descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet},
    device::Device,
    format::Format,
    image::view::ImageView,
    pipeline::{
        graphics::{
            input_assembly::InputAssemblyState,
            rasterization::{CullMode, RasterizationState},
            vertex_input::BuffersDefinition,
            viewport::ViewportState,
            GraphicsPipelineCreationError,
        },
        GraphicsPipeline, Pipeline, PipelineBindPoint, StateMode,
    },
    render_pass::{
        Framebuffer, FramebufferCreateInfo, RenderPass, RenderPassCreationError, Subpass,
    },
};

use crate::{
    data::vertex::Vertex,
    vulkan::{
        attachment_images::{AttachmentImageKeys, AttachmentImageMap, FrameBufferKeys},
        buffers::{IndexBufferKey, VertexBufferKey},
        init::VulkanState,
        pipeline::PipelineKeys,
        shaders::{
            flat,
            shader_loader::{LoadedShaders, ShaderKey},
        },
        view::get_ortho,
    },
};

use super::render_pass_loader::{RenderPassKeys, RenderPasses};

pub fn render_solid_draw_pass(
    builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
    state: &VulkanState,
) {
    if let (Some(vertex_buffer), Some(index_buffer)) = (
        state.vertex_buffers[VertexBufferKey::Flat].clone(),
        state.index_buffers[IndexBufferKey::Flat].clone(),
    ) {
        let uniform_buffer_subbuffer = {
            let uniform_data = flat::vs::ty::Data {
                view: get_ortho(state.swapchain.clone()).into(),
            };

            state.uniform_buffer.from_data(uniform_data).unwrap()
        };

        let set = PersistentDescriptorSet::new(
            &state.descriptor_set_allocator,
            state.pipelines[PipelineKeys::Solid]
                .layout()
                .set_layouts()
                .get(0)
                .unwrap()
                .clone(),
            [WriteDescriptorSet::buffer(0, uniform_buffer_subbuffer)],
        )
        .unwrap();

        builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    // clear_values: vec![Some([0.02, 0.02, 0.02, 1.0].into())],
                    ..RenderPassBeginInfo::framebuffer(
                        state.frame_buffers[FrameBufferKeys::Solid].clone(),
                    )
                },
                SubpassContents::Inline,
            )
            .unwrap()
            // .set_viewport(0, [viewport.clone()])
            .bind_pipeline_graphics(state.pipelines[PipelineKeys::Solid].clone())
            .bind_descriptor_sets(
                PipelineBindPoint::Graphics,
                state.pipelines[PipelineKeys::Solid].layout().clone(),
                0,
                set,
            )
            .bind_vertex_buffers(0, vertex_buffer)
            .bind_index_buffer(index_buffer.clone())
            .draw_indexed(index_buffer.len() as u32, 1, 0, 0, 0)
            .unwrap()
            .end_render_pass()
            .unwrap();
    }
}

pub fn create_framebuffer(
    render_passes: Arc<RenderPasses>,
    attachment_images: Arc<AttachmentImageMap>,
) -> Arc<Framebuffer> {
    let view = ImageView::new_default(attachment_images[AttachmentImageKeys::FinalOutput].clone())
        .unwrap();
    Framebuffer::new(
        render_passes[RenderPassKeys::Solid].clone(),
        FramebufferCreateInfo {
            attachments: vec![view],
            ..Default::default()
        },
    )
    .unwrap()
}

pub fn solid_draw_pass(
    device: Arc<Device>,
    format: Format,
) -> Result<Arc<RenderPass>, RenderPassCreationError> {
    vulkano::single_pass_renderpass!(
        device,
        attachments: {
            color: {
                load: Load,
                store: Store,
                format: format,
                samples: 1,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {}
        }
    )
}

pub fn solid_draw_pipeline(
    render_passes: Arc<RenderPasses>,
    device: Arc<Device>,
    shaders: Arc<LoadedShaders>,
) -> Result<Arc<GraphicsPipeline>, GraphicsPipelineCreationError> {
    let rasterization_state = RasterizationState {
        cull_mode: StateMode::Fixed(CullMode::None),
        ..Default::default()
    };

    GraphicsPipeline::start()
        .render_pass(Subpass::from(render_passes[RenderPassKeys::Solid].clone(), 0).unwrap())
        .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
        .input_assembly_state(
            InputAssemblyState::new().topology(
                vulkano::pipeline::graphics::input_assembly::PrimitiveTopology::TriangleList,
            ),
        )
        .vertex_shader(shaders[ShaderKey::FlatVs].entry_point("main").unwrap(), ())
        .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
        .rasterization_state(rasterization_state)
        .fragment_shader(shaders[ShaderKey::FlatFs].entry_point("main").unwrap(), ())
        .build(device)
}
