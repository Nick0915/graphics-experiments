use crate::graphics::*;

pub struct Grid {
    spacing: f32,
    height: f32,
    num_lines: u32,
    mesh: Mesh
}

impl Grid {
    pub fn new(spacing: f32, height: f32, num_lines: u32) -> Self {
        // #[rustfmt::skip]
        // let vertices = vec![
        //     // x,   y
        //     -1.0, 0.0,
        //      1.0, 0.0,
        // ];

        // let indices;

        // let mesh = Mesh::from_list(vertices, attr_layout, indices, shader_program)

        Self {
            spacing,
            height,
            num_lines,
            mesh: todo!(),
        };
    }
}
