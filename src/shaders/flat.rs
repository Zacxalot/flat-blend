pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "
            #version 450
            layout(location = 0) in vec2 position;

            layout(set = 0, binding = 0) uniform Data {
                mat4 view;
            } uniforms;

            void main() {
                gl_Position = uniforms.view * vec4(position, 0.0, 1.0);
            }
        ",
        types_meta: {
            use bytemuck::{Pod, Zeroable};

            #[derive(Clone, Copy, Zeroable, Pod)]
        }
    }
}

pub mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: "
            #version 450
            layout(location = 0) out vec4 f_color;
            void main() {
                f_color = vec4(0.6, 0.2, 0.8, 1.0);
            }
        "
    }
}
