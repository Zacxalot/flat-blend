use std::sync::Arc;

use vulkano::{
    device::Device,
    pipeline::{
        graphics::{
            input_assembly::InputAssemblyState,
            rasterization::{CullMode, RasterizationState},
            vertex_input::BuffersDefinition,
            viewport::ViewportState,
        },
        GraphicsPipeline, StateMode,
    },
    render_pass::{RenderPass, Subpass},
    shader::ShaderModule,
};

use crate::data::vertex::Vertex;

pub fn create_pipeline(
    render_pass: Arc<RenderPass>,
    vs: Arc<ShaderModule>,
    fs: Arc<ShaderModule>,
    device: Arc<Device>,
) -> Arc<GraphicsPipeline> {
    let rasterization_state = RasterizationState {
        cull_mode: StateMode::Fixed(CullMode::None),
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
