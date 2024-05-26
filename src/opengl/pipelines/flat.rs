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
    opengl::structs::{Mesh, Object},
};

pub struct FlatPipeline {
    pipeline: Pipeline,
    bindings: Bindings,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    objects: Arc<Vec<Object>>,
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
            objects: Arc::new(vec![]),
            projection_matrix,
            view_matrix,
        }
    }

    pub fn update(
        &mut self,
        ctx: &mut Context,
        objects: Arc<Vec<Object>>,
        meshes: Vec<Rc<RefCell<Mesh>>>,
    ) {
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

        self.objects = objects;
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);

        let projection_matrix = *(self.projection_matrix.lock().unwrap());
        let view_matrix = *(self.view_matrix.lock().unwrap());

        for object in &*self.objects {
            ctx.apply_uniforms(&shader::Uniforms {
                projection_matrix,
                view_matrix,
                translation: object.translation,
                colour: object.material.borrow().colour.into(),
            });

            ctx.draw(
                object.mesh.borrow().buffer_offset.try_into().unwrap(),
                (object.mesh.borrow().tris * 3).try_into().unwrap(),
                1,
            );
        }
    }
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;

    uniform mat4 view_matrix;
    uniform mat4 projection_matrix;
    uniform vec2 translation;
    
    varying lowp vec2 texcoord;
    
    void main() {
        gl_Position = projection_matrix * view_matrix * vec4(pos + translation, 0, 1);
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    uniform highp vec4 colour;

    void main() {
        gl_FragColor = colour;
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("view_matrix", UniformType::Mat4),
                    UniformDesc::new("projection_matrix", UniformType::Mat4),
                    UniformDesc::new("translation", UniformType::Float2),
                    UniformDesc::new("colour", UniformType::Float4),
                ],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub projection_matrix: glam::Mat4,
        pub view_matrix: glam::Mat4,
        pub translation: glam::Vec2,
        pub colour: [f32; 4],
    }
}
