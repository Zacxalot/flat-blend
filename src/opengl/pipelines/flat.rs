use miniquad::{
    Bindings, Buffer, BufferLayout, BufferType, Context, Pipeline, Shader, VertexAttribute,
    VertexFormat,
};

use crate::{
    data::vertex::{Index, Vertex},
    opengl::structs::{Object, RenderObject},
};

pub struct FlatPipeline {
    pipeline: Pipeline,
    bindings: Bindings,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    render_objects: Vec<RenderObject>,
}

impl FlatPipeline {
    pub fn new(ctx: &mut Context) -> FlatPipeline {
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

        let render_objects: Vec<RenderObject> = vec![];

        FlatPipeline {
            pipeline,
            bindings,
            index_buffer,
            vertex_buffer,
            render_objects,
        }
    }

    pub fn update(&mut self, ctx: &mut Context, objects: &[Object]) {
        let mut vertices: Vec<Vertex> = vec![];
        let mut indices: Vec<Index> = vec![];
        let mut render_objects: Vec<RenderObject> = vec![];

        objects.iter().for_each(|object| {
            render_objects.push(RenderObject {
                tris: object.indices.len() as Index / 3,
                index_offset: indices.len() as Index,
            });

            vertices.extend_from_slice(&object.vertices);
            indices.extend_from_slice(&object.indices);
        });

        println!("Vertices: {:?}", vertices);
        println!("Indices: {:?}", indices);

        self.vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        self.index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        self.bindings = Bindings {
            vertex_buffers: vec![self.vertex_buffer],
            index_buffer: self.index_buffer,
            images: vec![],
        };
        self.render_objects = render_objects;
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        ctx.apply_uniforms(&shader::Uniforms { offset: (0.5, 0.5) });

        for render_object in &self.render_objects {
            ctx.draw(
                render_object.index_offset.try_into().unwrap(),
                (render_object.tris * 3).try_into().unwrap(),
                1,
            );
        }
    }
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;

    uniform vec2 offset;

    varying lowp vec2 texcoord;

    void main() {
        gl_Position = vec4(pos + offset, 0, 1);
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    void main() {
        gl_FragColor = vec4(1, 0, 1, 1);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("offset", UniformType::Float2)],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub offset: (f32, f32),
    }
}
