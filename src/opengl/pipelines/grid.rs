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
    zoom: Arc<Mutex<f32>>,
}

impl GridPipeline {
    pub fn new(
        ctx: &mut Context,
        position: Arc<Mutex<Vec2>>,
        zoom: Arc<Mutex<f32>>,
    ) -> GridPipeline {
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
            zoom,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        let position = *(self.position.lock().unwrap());
        let zoom = *(self.zoom.lock().unwrap());
        ctx.apply_uniforms(&shader::Uniforms {
            u_resolution: ctx.screen_size().into(),
            u_position: position,
            u_zoom: zoom,
            u_square_size: 100.0,
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
    #extension GL_OES_standard_derivatives : enable
    precision highp float;

    uniform vec2 u_resolution;
    uniform vec2 u_position;
    uniform float u_zoom;
    uniform float u_square_size;

    float getGrid(vec2 uv, float size) {
        // Line thickness in screen pixels
        float lineWidth = 1.5;

        // Normalize coordinates immediately to avoid precision issues
        // fract returns the fractional part, keeping values in [0, 1)
        vec2 coord = fract(uv / size);

        // Distance to nearest grid line (either at 0 or 1)
        vec2 distToLine = min(coord, 1.0 - coord);
        float dist = min(distToLine.x, distToLine.y);

        // Use derivatives to get pixel-space distance
        vec2 derivative = fwidth(coord);
        float pixelDist = dist / max(derivative.x, derivative.y);

        return 1.0 - smoothstep(0.0, lineWidth, pixelDist);
    }

    float getAxis(vec2 uv, int axis) {
        // Axis line thickness in screen pixels
        float lineWidth = 6.0;

        float dist = abs(uv[axis] + 0.5);

        // Use derivatives to get pixel-space distance
        float derivative = axis == 0 ? fwidth(uv.x) : fwidth(uv.y);
        float pixelDist = dist / derivative;

        return 1.0 - smoothstep(0.0, lineWidth, pixelDist);
    }

    void main() {
        // Calculate LOD level based on zoom
        // When zoom is 1.0, we want base size
        // When zoom is 0.5, squares appear half size -> need to double
        // When zoom is 2.0, squares appear double size -> can halve
        float baseSquareSize = u_square_size;

        // Calculate screen-space size of a square
        float screenSquareSize = baseSquareSize * u_zoom;

        // Target screen size for squares (in pixels)
        float targetScreenSize = 80.0;

        // Calculate how many times we need to double/halve
        float lodLevel = floor(log2(screenSquareSize / targetScreenSize));

        // Apply LOD scaling
        float lodScale = pow(2.0, -lodLevel);
        float squareSize = baseSquareSize * lodScale;
        float smallSquareSize = squareSize / 2.0;

        // Convert screen space to world space centered at screen center
        vec2 screenCenter = u_resolution / 2.0;
        vec2 uv = (gl_FragCoord.xy - screenCenter - u_position.xy) / u_zoom;

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
                    UniformDesc::new("u_zoom", UniformType::Float1),
                    UniformDesc::new("u_square_size", UniformType::Float1),
                ],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub u_resolution: glam::Vec2,
        pub u_position: glam::Vec2,
        pub u_zoom: f32,
        pub u_square_size: f32,
    }
}
