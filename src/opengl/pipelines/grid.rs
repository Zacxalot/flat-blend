use std::sync::{Arc, Mutex};

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
    position: Arc<Mutex<Vec2>>,
}

impl GridPipeline {
    pub fn new(ctx: &mut Context, position: Arc<Mutex<Vec2>>) -> GridPipeline {
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
            position,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        let position = *(self.position.lock().unwrap());
        ctx.apply_uniforms(&shader::Uniforms {
            u_resolution: ctx.screen_size().into(),
            u_position: position,
        });

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

    pub const FRAGMENT: &str = r#"
    #version 100
    precision mediump float;

    uniform vec2 u_resolution;
    uniform vec2 u_position;

    int squareSize = 160;

    float getGrid(vec2 uv, int size) {
        vec2 grid = mod((uv - (u_resolution / 2.0)) - 0.5,float(size));
        return 1.0 - (clamp(min(grid.x, grid.y), 1.0, 2.0) - 1.0);
    }

    float getAxis(vec2 uv, int axis) {
        float line = abs(((uv[axis] + 0.5) - (u_resolution[axis]/2.0))/4.0);
        return clamp(1.0 - line, 0.0, 1.0);
    }

    void main() {
        int smallSquareSize = squareSize / 2;
        vec2 uv = gl_FragCoord.xy + u_position.xy * -320.0;

        float big = getGrid(uv, squareSize);
        float small = getGrid(uv, smallSquareSize);
        float xAxis = getAxis(uv, 1);
        float yAxis = getAxis(uv, 0);
        vec3 axis = vec3(xAxis, yAxis, 0.0);

        vec3 grid = vec3(max(big / 2.0, small / 8.0));
        vec3 gridCol = vec3(grid);
        float mask = max(axis.x, axis.y);
        gl_FragColor = vec4(mix(grid , axis, mask), 1.0);
    }
    "#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("u_resolution", UniformType::Float2),
                    UniformDesc::new("u_position", UniformType::Float2),
                ],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub u_resolution: glam::Vec2,
        pub u_position: glam::Vec2,
    }
}
