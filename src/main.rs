use lyon::{
    geom::{
        euclid::{Point2D, UnknownUnit},
        point, Transform,
    },
    lyon_tessellation::{
        geometry_builder::simple_builder, FillOptions, FillTessellator, VertexBuffers,
    },
    math::{Point, Vector},
    path::Path,
};
use miniquad::{
    conf, Bindings, Buffer, BufferLayout, BufferType, Context, EventHandler, MouseButton, Pipeline,
    Shader, VertexAttribute, VertexFormat,
};

struct Stage {
    pipeline: Pipeline,
    bindings: Bindings,
    start_time: f64,
    last_frame: f64,
    uniforms: shader::Uniforms,
}

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}

impl From<Point2D<f32, UnknownUnit>> for Vec2 {
    fn from(Point2D { x, y, .. }: Point2D<f32, UnknownUnit>) -> Self {
        Vec2 { x, y }
    }
}

#[repr(C)]
struct Vertex {
    pos: Vec2,
}

impl From<Vec2> for Vertex {
    fn from(pos: Vec2) -> Self {
        Vertex { pos }
    }
}

impl Stage {
    pub fn new(ctx: &mut Context, vertices: Vec<Vertex>, indices: Vec<u16>) -> Stage {
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);
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

        let uniforms = shader::Uniforms {
            time: 0.,
            blobs_count: 1,
            blobs_positions: [(0., 0.); 32],
        };

        let time = miniquad::date::now();

        Stage {
            pipeline,
            bindings,
            start_time: time,
            uniforms,
            last_frame: time,
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        self.uniforms.time = (miniquad::date::now() - self.start_time) as f32;

        ctx.begin_default_pass(Default::default());
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        ctx.apply_uniforms(&self.uniforms);
        ctx.draw(0, 6, 1);
        ctx.end_render_pass();

        ctx.commit_frame();
    }
}

fn build_path() -> Path {
    let mut path_builder = Path::builder();
    path_builder.begin(point(0.0, 0.0));
    path_builder.line_to(point(0.5, 1.0));
    path_builder.line_to(point(1.0, 0.0));
    path_builder.line_to(point(0.5, 0.5));
    path_builder.end(true);
    path_builder.build()
}

fn tesselate_path(path: &Path) -> VertexBuffers<Point, u16> {
    let mut buffers: VertexBuffers<Point, u16> = VertexBuffers::new();

    {
        let mut vertex_builder = simple_builder(&mut buffers);

        // Create the tessellator.
        let mut tessellator = FillTessellator::new();

        // Compute the tessellation.

        tessellator
            .tessellate_path(path, &FillOptions::default(), &mut vertex_builder)
            .unwrap();
    }

    buffers
}

fn main() {
    let path = build_path();
    let buffers = tesselate_path(&path);

    let vertices = buffers
        .vertices
        .iter()
        .map(|vertex| Vertex::from(Vec2::from(*vertex)))
        .collect::<Vec<Vertex>>();

    let indices = buffers.indices.to_vec();

    for vert in buffers.vertices.iter() {
        println!("{:?}", vert);
    }

    miniquad::start(conf::Conf::default(), |ctx| {
        Box::new(Stage::new(ctx, vertices, indices))
    });
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;
    attribute vec2 uv;
    varying highp vec2 texcoord;
    void main() {
        gl_Position = vec4(pos, 0, 1);
        texcoord = uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    precision highp float;
    varying vec2 texcoord;
    
    void main() {
        gl_FragColor = vec4( 1.0,0.5,1.0, 1.0 );
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("time", UniformType::Float1),
                    UniformDesc::new("blobs_count", UniformType::Int1),
                    UniformDesc::new("blobs_positions", UniformType::Float2).array(32),
                ],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub time: f32,
        pub blobs_count: i32,
        pub blobs_positions: [(f32, f32); 32],
    }
}
