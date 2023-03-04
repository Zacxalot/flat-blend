use std::sync::Arc;

use vulkano::{
    device::Device,
    format::Format,
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
    render_pass::{RenderPass, RenderPassCreationError, Subpass},
};

use crate::{
    data::vertex::Vertex,
    vulkan::shaders::shader_loader::{LoadedShaders, ShaderKey},
};

pub fn solid_draw_pass(
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

pub fn solid_draw_pipeline(
    render_pass: Arc<RenderPass>,
    device: Arc<Device>,
    shaders: Arc<LoadedShaders>,
) -> Result<Arc<GraphicsPipeline>, GraphicsPipelineCreationError> {
    let rasterization_state = RasterizationState {
        cull_mode: StateMode::Fixed(CullMode::None),
        ..Default::default()
    };

    GraphicsPipeline::start()
        .render_pass(Subpass::from(render_pass, 0).unwrap())
        .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
        .input_assembly_state(
            InputAssemblyState::new().topology(
                vulkano::pipeline::graphics::input_assembly::PrimitiveTopology::TriangleList,
            ),
        )
        .vertex_shader(
            shaders
                .get(&ShaderKey::FlatVs)
                .unwrap()
                .entry_point("main")
                .unwrap(),
            (),
        )
        .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
        .rasterization_state(rasterization_state)
        .fragment_shader(
            shaders
                .get(&ShaderKey::FlatFs)
                .unwrap()
                .entry_point("main")
                .unwrap(),
            (),
        )
        .build(device)
}
