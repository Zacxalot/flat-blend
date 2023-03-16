use std::sync::Arc;

use vulkano::{
    buffer::TypedBufferAccess,
    command_buffer::{
        AutoCommandBufferBuilder, PrimaryAutoCommandBuffer, RenderPassBeginInfo, SubpassContents,
    },
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
        GraphicsPipeline, StateMode,
    },
    render_pass::{
        Framebuffer, FramebufferCreateInfo, RenderPass, RenderPassCreationError, Subpass,
    },
};

use crate::{
    data::vertex::Vertex,
    vulkan::{
        attachment_images::{AttachmentImageKeys, AttachmentImageMap, FrameBufferKeys},
        buffers::VertexBufferKey,
        init::VulkanState,
        pipeline::PipelineKeys,
        shaders::{
            shader_loader::{LoadedShaders, ShaderKey},
        },
    },
};

use super::render_pass_loader::{RenderPassKeys, RenderPasses};

pub fn render_grid_draw_pass(
    builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
    state: &VulkanState,
) {
    if let Some(vertex_buffer) = state.vertex_buffers[VertexBufferKey::Grid].clone() {
        builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some([0.02, 0.02, 0.02, 1.0].into())],
                    ..RenderPassBeginInfo::framebuffer(
                        state.frame_buffers[FrameBufferKeys::Grid].clone(),
                    )
                },
                SubpassContents::Inline,
            )
            .unwrap()
            .set_viewport(0, [state.viewport.clone()])
            .bind_pipeline_graphics(state.pipelines[PipelineKeys::Grid].clone())
            .bind_vertex_buffers(0, vertex_buffer.clone())
            .draw(vertex_buffer.len() as u32, 1, 0, 0)
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
        render_passes[RenderPassKeys::Grid].clone(),
        FramebufferCreateInfo {
            attachments: vec![view],
            ..Default::default()
        },
    )
    .unwrap()
}

pub fn grid_draw_pass(
    device: Arc<Device>,
    format: Format,
) -> Result<Arc<RenderPass>, RenderPassCreationError> {
    vulkano::single_pass_renderpass!(
        device,
        attachments: {
            color: {
                load: Clear,
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

pub fn grid_draw_pipeline(
    render_passes: Arc<RenderPasses>,
    device: Arc<Device>,
    shaders: Arc<LoadedShaders>,
) -> Result<Arc<GraphicsPipeline>, GraphicsPipelineCreationError> {
    let rasterization_state = RasterizationState {
        cull_mode: StateMode::Fixed(CullMode::None),
        ..Default::default()
    };

    GraphicsPipeline::start()
        .render_pass(Subpass::from(render_passes[RenderPassKeys::Grid].clone(), 0).unwrap())
        .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
        .input_assembly_state(
            InputAssemblyState::new().topology(
                vulkano::pipeline::graphics::input_assembly::PrimitiveTopology::TriangleList,
            ),
        )
        .vertex_shader(shaders[ShaderKey::GridVs].entry_point("main").unwrap(), ())
        .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
        .rasterization_state(rasterization_state)
        .fragment_shader(shaders[ShaderKey::GridFs].entry_point("main").unwrap(), ())
        .build(device)
}
