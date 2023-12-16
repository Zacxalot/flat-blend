use glam::Vec2;
use miniquad::{
    Bindings, Buffer, BufferLayout, BufferType, Context, Pipeline, Shader, VertexAttribute,
    VertexFormat,
};

use crate::data::vertex::{Index, Vertex};

pub struct GridPipeline {
    pipeline: Pipeline,
    bindings: Bindings,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

impl GridPipeline {
    pub fn new(ctx: &mut Context) -> GridPipeline {
        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [Vertex{pos: Vec2::new(-1.0, -1.0)},Vertex{pos: Vec2::new(1.0, -1.0)},Vertex{pos: Vec2::new(-1.0, 1.0)},Vertex{pos: Vec2::new(1.0, 1.0)}];
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);
        let indices: [Index; 6] = [0, 1, 2, 1, 2, 3];
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

        GridPipeline {
            pipeline,
            bindings,
            index_buffer,
            vertex_buffer,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);

        // let projection_matrix = *(self.projection_matrix.lock().unwrap());
        // let view_matrix = *(self.view_matrix.lock().unwrap());

        // ctx.apply_uniforms(&shader::Uniforms {
        //     projection_matrix,
        //     view_matrix,
        // });

        ctx.draw(0, 6, 1);
    }
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;

    void main() {
        gl_Position = vec4(pos, 0, 1);
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    void main() {
        gl_FragColor = vec4(0, 1, 0, 1);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout { uniforms: vec![] },
        }
    }

    #[repr(C)]
    pub struct Uniforms {}
}
