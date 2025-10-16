use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use glam::Mat4;
use miniquad::{
    Bindings, Buffer, BufferLayout, BufferType, Context, Pipeline, Shader, VertexAttribute,
    VertexFormat,
};

use crate::{
    data::vertex::{Index, Vertex},
    opengl::{
        scene::{ObjectKey, SceneData},
        structs::Mesh,
    },
};

pub struct FlatPipeline {
    pipeline: Pipeline,
    bindings: Bindings,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    projection_matrix: Arc<Mutex<Mat4>>,
    view_matrix: Arc<Mutex<Mat4>>,
}

impl FlatPipeline {
    pub fn new(
        ctx: &mut Context,
        projection_matrix: Arc<Mutex<Mat4>>,
        view_matrix: Arc<Mutex<Mat4>>,
    ) -> FlatPipeline {
        #[rustfmt::skip]
        let vertices: [Vertex; 0] = [];
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);
        let indices: [Index; 0] = [];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![],
        };

        let shader = Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::meta()).unwrap();

        let pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[VertexAttribute::new("pos", VertexFormat::Float2)],
            shader,
        );

        FlatPipeline {
            pipeline,
            bindings,
            index_buffer,
            vertex_buffer,
            projection_matrix,
            view_matrix,
        }
    }

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

        self.bindings = Bindings {
            vertex_buffers: vec![self.vertex_buffer],
            index_buffer: self.index_buffer,
            images: vec![],
        };
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        scene_data: &SceneData,
        projection_matrix: Mat4,
        view_matrix: Mat4,
    ) {
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);

        let objects = scene_data.objects();
        let visible_keys = scene_data.visible_objects();

        // Only draw visible objects
        for &key in visible_keys {
            let object = &objects[key];

            ctx.apply_uniforms(&shader::Uniforms {
                model_matrix: object.get_model_matrix(),
                view_matrix,
                projection_matrix,
                colour: object.borrow_material().colour.into(),
                selected: if object.selected { 1.0 } else { 0.0 },
            });

            ctx.draw(
                object.borrow_mesh().buffer_offset.try_into().unwrap(),
                (object.borrow_mesh().tris * 3).try_into().unwrap(),
                1,
            );
        }
    }
}

mod shader {
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
    uniform highp vec4 colour;
    uniform lowp float selected;

    void main() {
        if (selected > 0.5) {
            // Brighten selected objects by mixing with white
            gl_FragColor = mix(colour, vec4(1.0, 1.0, 1.0, 1.0), 0.4);
        } else {
            gl_FragColor = colour;
        }
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("model_matrix", UniformType::Mat4),
                    UniformDesc::new("view_matrix", UniformType::Mat4),
                    UniformDesc::new("projection_matrix", UniformType::Mat4),
                    UniformDesc::new("colour", UniformType::Float4),
                    UniformDesc::new("selected", UniformType::Float1),
                ],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub model_matrix: glam::Mat4,
        pub view_matrix: glam::Mat4,
        pub projection_matrix: glam::Mat4,
        pub colour: [f32; 4],
        pub selected: f32,
    }
}
