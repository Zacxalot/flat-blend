pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "
            #version 450
            layout(location = 0) in vec2 position;


            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
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
            layout(location = 0) out vec4 f_colour;

            int squareSize = 160;
            int smallSquareSize = squareSize / 2;

            vec2 uResolution = vec2(1000,600);

            float getGrid(vec2 uv,int size) {
                vec2 grid = mod((uv - (uResolution / 2)) - 0.5,size);
                return 1.0 - (clamp(min(grid.x, grid.y), 1.0, 2.0) - 1.0);
            }

            float getAxis(vec2 uv, int axis) {
                float line = abs(((uv[axis] + 0.5) - (uResolution[axis]/2))/4);
                return clamp(1.0 - line, 0.0, 1.0);
            }

            void main() {
                float big = getGrid(gl_FragCoord.xy, squareSize);
                float small = getGrid(gl_FragCoord.xy, smallSquareSize);
                float xAxis = getAxis(gl_FragCoord.xy, 1);
                float yAxis = getAxis(gl_FragCoord.xy, 0);
                vec3 axis = vec3(xAxis, yAxis, 0.0);
                

                vec3 grid = vec3(max(big / 2, small / 8));
                vec3 gridCol = vec3(grid);
                float mask = max(axis.x, axis.y);
	            f_colour = vec4(mix(grid , axis, mask), 0.0);
            }
        "
    }
}
