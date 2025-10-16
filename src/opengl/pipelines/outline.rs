//! Outline Pipeline for 2D Object Selection
//!
//! This pipeline generates outlines for selected objects using a two-pass rendering approach
//! inspired by Blender's outline system, adapted for 2D space.
//!
//! ## Algorithm Overview
//!
//! **Pass 1 - ID Buffer Generation:**
//! - Renders selected objects to an off-screen texture
//! - Each object is rendered with a unique color ID
//! - This creates a clean ID buffer where object boundaries are well-defined
//!
//! **Pass 2 - Edge Detection & Outline:**
//! - Applies edge detection on the ID buffer using a simple kernel
//! - Detects boundaries where object IDs change or meet empty space
//! - Outputs a transparent image with colored outline
//! - Uses alpha blending to composite over the scene
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! // In RenderContext::new()
//! let (width, height) = ctx.screen_size();
//! let outline_pipeline = OutlinePipeline::new(
//!     ctx,
//!     projection_matrix.clone(),
//!     view_matrix.clone(),
//!     width as u32,
//!     height as u32,
//! );
//!
//! // Update mesh data (same as other pipelines)
//! outline_pipeline.update(ctx, meshes);
//!
//! // In resize event:
//! outline_pipeline.resize(ctx, width as u32, height as u32);
//!
//! // In draw loop (after drawing regular objects):
//! outline_pipeline.draw(ctx, &scene_data, projection_matrix, view_matrix);
//! ```
//!
//! ## 2D Simplifications vs Blender
//!
//! Blender's 3D outline system also uses:
//! - Depth buffer for z-ordering and occlusion
//! - Multiple outline styles (silhouette, edge, etc.)
//! - Stencil buffer for complex masking
//!
//! For 2D, we simplify:
//! - No depth buffer needed (2D has natural painter's algorithm ordering)
//! - Single outline style (boundary detection)
//! - Direct alpha blending instead of complex compositing
//!
//! ## Performance Characteristics
//!
//! - ID Pass: O(n) where n = number of selected object triangles
//! - Outline Pass: O(p) where p = number of screen pixels
//! - Memory: One RGBA8 texture at screen resolution
//! - Bandwidth: Two full-screen passes per frame (only when objects are selected)

use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use glam::{Mat4, Vec2};
use miniquad::{
    Bindings, Buffer, BufferLayout, BufferType, Context, PassAction, Pipeline, RenderPass, Shader,
    Texture, TextureFormat, TextureParams, VertexAttribute, VertexFormat,
};

use crate::{
    data::vertex::{Index, Vertex},
    opengl::{scene::SceneData, structs::Mesh},
};

/// Pipeline for rendering object outlines using edge detection
///
/// This pipeline uses a two-pass approach:
/// 1. ID Pass: Renders objects with unique ID colors to a texture
/// 2. Outline Pass: Applies edge detection to find object boundaries
pub struct OutlinePipeline {
    // ID rendering pass (renders to texture)
    id_pipeline: Pipeline,
    id_bindings: Bindings,
    id_render_pass: RenderPass,
    id_texture: Texture,

    // Outline generation pass (renders to screen)
    outline_pipeline: Pipeline,
    outline_bindings: Bindings,

    // Shared buffers
    vertex_buffer: Buffer,
    index_buffer: Buffer,

    // Full-screen quad for outline pass
    quad_vertex_buffer: Buffer,
    quad_index_buffer: Buffer,

    // Matrices
    projection_matrix: Arc<Mutex<Mat4>>,
    view_matrix: Arc<Mutex<Mat4>>,

    // Screen dimensions (for texture sizing)
    width: u32,
    height: u32,
}

impl OutlinePipeline {
    pub fn new(
        ctx: &mut Context,
        projection_matrix: Arc<Mutex<Mat4>>,
        view_matrix: Arc<Mutex<Mat4>>,
        width: u32,
        height: u32,
    ) -> Self {
        // Create ID texture (RGBA8 format for object IDs)
        let id_texture = Texture::new_render_texture(
            ctx,
            TextureParams {
                width,
                height,
                format: TextureFormat::RGBA8,
                ..Default::default()
            },
        );

        let id_render_pass = RenderPass::new(ctx, id_texture, None);

        // Initialize empty vertex and index buffers for objects
        let vertices: [Vertex; 0] = [];
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);
        let indices: [Index; 0] = [];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        // Create full-screen quad for outline pass
        #[rustfmt::skip]
        let quad_vertices: [Vertex; 4] = [
            Vertex { pos: Vec2::new(-1.0, -1.0) },
            Vertex { pos: Vec2::new(1.0, -1.0) },
            Vertex { pos: Vec2::new(-1.0, 1.0) },
            Vertex { pos: Vec2::new(1.0, 1.0) },
        ];
        let quad_vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &quad_vertices);
        let quad_indices: [Index; 6] = [0, 1, 2, 1, 2, 3];
        let quad_index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &quad_indices);

        // Create ID pass bindings
        let id_bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![],
        };

        // Create outline pass bindings (includes the ID texture)
        let outline_bindings = Bindings {
            vertex_buffers: vec![quad_vertex_buffer],
            index_buffer: quad_index_buffer,
            images: vec![id_texture],
        };

        // Create shaders and pipelines
        let id_shader = Shader::new(
            ctx,
            id_shader::VERTEX,
            id_shader::FRAGMENT,
            id_shader::meta(),
        )
        .unwrap();

        let id_pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[VertexAttribute::new("pos", VertexFormat::Float2)],
            id_shader,
        );

        let outline_shader = Shader::new(
            ctx,
            outline_shader::VERTEX,
            outline_shader::FRAGMENT,
            outline_shader::meta(),
        )
        .unwrap();

        let outline_pipeline = Pipeline::with_params(
            ctx,
            &[BufferLayout::default()],
            &[VertexAttribute::new("pos", VertexFormat::Float2)],
            outline_shader,
            miniquad::PipelineParams {
                color_blend: Some(miniquad::BlendState::new(
                    miniquad::Equation::Add,
                    miniquad::BlendFactor::Value(miniquad::BlendValue::SourceAlpha),
                    miniquad::BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
                )),
                ..Default::default()
            },
        );

        Self {
            id_pipeline,
            id_bindings,
            id_render_pass,
            id_texture,
            outline_pipeline,
            outline_bindings,
            vertex_buffer,
            index_buffer,
            quad_vertex_buffer,
            quad_index_buffer,
            projection_matrix,
            view_matrix,
            width,
            height,
        }
    }

    /// Update the mesh data for rendering
    pub fn update(&mut self, ctx: &mut Context, meshes: Vec<Rc<RefCell<Mesh>>>) {
        let mut vertices: Vec<Vertex> = vec![];
        let mut indices: Vec<Index> = vec![];

        meshes.iter().for_each(|mesh| {
            let (mesh_vertices, mesh_indices) = mesh.borrow_mut().update(indices.len() as Index);
            vertices.extend_from_slice(&mesh_vertices);
            indices.extend_from_slice(&mesh_indices);
        });

        self.vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);
        self.index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        self.id_bindings = Bindings {
            vertex_buffers: vec![self.vertex_buffer],
            index_buffer: self.index_buffer,
            images: vec![],
        };
    }

    /// Resize the render textures when the window size changes
    pub fn resize(&mut self, ctx: &mut Context, width: u32, height: u32) {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;

            // Delete old render pass first (it references the texture)
            self.id_render_pass.delete(ctx);

            // Now delete and recreate the texture
            self.id_texture.delete();
            self.id_texture = Texture::new_render_texture(
                ctx,
                TextureParams {
                    width,
                    height,
                    format: TextureFormat::RGBA8,
                    ..Default::default()
                },
            );

            // Create new render pass with new texture
            self.id_render_pass = RenderPass::new(ctx, self.id_texture, None);

            // Update bindings with new texture
            self.id_bindings = Bindings {
                vertex_buffers: vec![self.vertex_buffer],
                index_buffer: self.index_buffer,
                images: vec![],
            };

            self.outline_bindings = Bindings {
                vertex_buffers: vec![self.quad_vertex_buffer],
                index_buffer: self.quad_index_buffer,
                images: vec![self.id_texture],
            };
        }
    }

    /// Draw the outline for selected objects
    pub fn draw(
        &mut self,
        ctx: &mut Context,
        scene_data: &SceneData,
        projection_matrix: Mat4,
        view_matrix: Mat4,
    ) {
        let objects = scene_data.objects();
        let selected_indices = scene_data.visible_selected_objects();

        // If no objects are selected, skip rendering
        if selected_indices.is_empty() {
            return;
        }

        // Pass 1: Render selected objects to ID texture
        ctx.begin_pass(
            self.id_render_pass,
            PassAction::clear_color(0.0, 0.0, 0.0, 0.0),
        );

        ctx.apply_pipeline(&self.id_pipeline);
        ctx.apply_bindings(&self.id_bindings);

        // Render only selected objects with their unique IDs
        for (id_index, &obj_index) in selected_indices.iter().enumerate() {
            let object = &objects[obj_index];

            // Convert object index to a unique color ID
            // Use (id_index + 1) so we don't use pure black (0,0,0) as an ID
            let id = id_index + 1;
            let r = ((id >> 16) & 0xFF) as f32 / 255.0;
            let g = ((id >> 8) & 0xFF) as f32 / 255.0;
            let b = (id & 0xFF) as f32 / 255.0;

            ctx.apply_uniforms(&id_shader::Uniforms {
                model_matrix: object.get_model_matrix(),
                view_matrix,
                projection_matrix,
                object_id: [r, g, b, 1.0],
            });

            ctx.draw(
                object.borrow_mesh().buffer_offset.try_into().unwrap(),
                (object.borrow_mesh().tris * 3).try_into().unwrap(),
                1,
            );
        }

        ctx.end_render_pass();

        // Pass 2: Apply edge detection to generate outline
        // Note: This renders to the current render target (usually the screen)
        ctx.apply_pipeline(&self.outline_pipeline);
        ctx.apply_bindings(&self.outline_bindings);

        ctx.apply_uniforms(&outline_shader::Uniforms {
            texture_size: [self.width as f32, self.height as f32],
            outline_color: [1.0, 0.5, 0.0, 1.0], // Orange outline
            outline_width: 2.0,
        });

        ctx.draw(0, 6, 1);
    }
}

/// Shader for rendering objects with unique ID colors
mod id_shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;

    uniform mat4 model_matrix;
    uniform mat4 view_matrix;
    uniform mat4 projection_matrix;

    void main() {
        gl_Position = projection_matrix * view_matrix * model_matrix * vec4(pos, 0, 1);
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    precision highp float;

    uniform vec4 object_id;

    void main() {
        gl_FragColor = object_id;
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("model_matrix", UniformType::Mat4),
                    UniformDesc::new("view_matrix", UniformType::Mat4),
                    UniformDesc::new("projection_matrix", UniformType::Mat4),
                    UniformDesc::new("object_id", UniformType::Float4),
                ],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub model_matrix: glam::Mat4,
        pub view_matrix: glam::Mat4,
        pub projection_matrix: glam::Mat4,
        pub object_id: [f32; 4],
    }
}

/// Shader for edge detection and outline generation
mod outline_shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;
    varying vec2 uv;

    void main() {
        // Convert from [-1, 1] to [0, 1] for texture coordinates
        uv = (pos + 1.0) / 2.0;
        gl_Position = vec4(pos, 0, 1);
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    precision highp float;

    varying vec2 uv;
    uniform sampler2D id_texture;
    uniform vec2 texture_size;
    uniform vec4 outline_color;
    uniform float outline_width;

    void main() {
        // Calculate texel size
        vec2 texel_size = 1.0 / texture_size;

        // Sample the center pixel
        vec4 center = texture2D(id_texture, uv);

        // If the center pixel has no object (alpha = 0), check neighbors for edges
        if (center.a < 0.5) {
            // Use a simple 3x3 kernel for edge detection
            // Check 8 neighbors around the current pixel
            float edge = 0.0;

            for (float x = -outline_width; x <= outline_width; x += 1.0) {
                for (float y = -outline_width; y <= outline_width; y += 1.0) {
                    if (x == 0.0 && y == 0.0) continue;

                    vec2 offset = vec2(x, y) * texel_size;
                    vec4 neighbor = texture2D(id_texture, uv + offset);

                    // If any neighbor has an object, we're on an edge
                    if (neighbor.a > 0.5) {
                        edge = 1.0;
                        break;
                    }
                }
                if (edge > 0.5) break;
            }

            // Output the outline color with appropriate alpha
            gl_FragColor = vec4(outline_color.rgb, outline_color.a * edge);
        } else {
            // Inside an object, check if we're on an internal edge
            // (adjacent to a different object or empty space)
            float edge = 0.0;

            for (float x = -1.0; x <= 1.0; x += 1.0) {
                for (float y = -1.0; y <= 1.0; y += 1.0) {
                    if (x == 0.0 && y == 0.0) continue;

                    vec2 offset = vec2(x, y) * texel_size;
                    vec4 neighbor = texture2D(id_texture, uv + offset);

                    // Check if neighbor is different (different ID or empty)
                    if (distance(neighbor.rgb, center.rgb) > 0.01 || neighbor.a < 0.5) {
                        edge = 1.0;
                        break;
                    }
                }
                if (edge > 0.5) break;
            }

            // Don't render outline inside the object
            gl_FragColor = vec4(0.0, 0.0, 0.0, 0.0);
        }
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["id_texture".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("texture_size", UniformType::Float2),
                    UniformDesc::new("outline_color", UniformType::Float4),
                    UniformDesc::new("outline_width", UniformType::Float1),
                ],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub texture_size: [f32; 2],
        pub outline_color: [f32; 4],
        pub outline_width: f32,
    }
}
