use glam::*;

use crate::{
    camera::{Camera, FocusCamera3D},
    graphics::*,
    util,
};

pub struct Grid10x10 {
    spacing: f32,
    height: f32,
    shader_program: ShaderProgram,
    vertex_array: VAO,
    vertex_buffer: BufferObject,
}

impl Drawable for Grid10x10 {
    fn draw<T: Camera>(&self, camera: &T) {
        let mut mvp = camera.projection_mat() * camera.view_mat();
        self.shader_program
            .uniform_matrix4f("u_MVP", &mvp.to_cols_array()[0]);

        // bind all appropriate buffers/arrays/programs
        self.vertex_array.bind();
        self.vertex_buffer.bind();
        self.shader_program.r#use();

        // todo!()
        unsafe {
            gl::DrawArrays(gl::LINES, 0, 2);
        }
    }
}

impl Grid10x10 {
    pub fn new(spacing: f32, height: f32) -> Self {
        #[rustfmt::skip]
        let vertices: Vec<f32> = vec![
            // x,      y,   z
             0.0, height, 0.0,
             1.0, height, 0.0,
        ];

        let attr_layout: Vec<u32> = vec![3];

        // 1. create VAO, VBO, IBO
        let mut vertex_array = VAO::new(gl::FLOAT);
        let mut vertex_buffer = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
        // let mut index_buffer = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);

        // 2. bind VAO
        vertex_array.bind();

        // 3. bind IBO and populate
        // index_buffer.bind();
        // index_buffer.buffer_data(&indices);

        // 4. bind VBO and populate
        vertex_buffer.bind();
        vertex_buffer.buffer_data(&vertices);

        // 5. set VAO layout
        vertex_array.set_attr_layout::<f32>(attr_layout);

        Self {
            spacing,
            height,
            shader_program: ShaderProgram::new(
                include_str!("../../shader/grid.vert"),
                include_str!("../../shader/main.frag"),
            ),
            vertex_array,
            vertex_buffer,
        }
    }

    const NUM_HORIZ_LINES: i32 = 10;
    const NUM_VERT_LINES: i32 = 10;

    pub fn generate_mvps(&self) -> Vec<glam::Mat4> {
        let mut mvps: Vec<glam::Mat4> = Vec::new();

        let horiz_len = (Self::NUM_VERT_LINES - 1) as f32;
        let vert_len = (Self::NUM_HORIZ_LINES - 1) as f32;

        for i in 0..Self::NUM_HORIZ_LINES {
            let x_start = (i - Self::NUM_HORIZ_LINES / 2) as f32;
            let y_start = (-Self::NUM_HORIZ_LINES / 2) as f32;

            let mat = glam::Mat4::from_scale_rotation_translation(
                Vec3::new(self.spacing, 0., self.spacing),
                Quat::IDENTITY,
                Vec3::new(x_start, 0., y_start),
            );

            mvps.push(mat);
            println!(
                "Horizontal line ({}, 0, {}) to ({}, 0, {})",
                x_start * self.spacing,
                y_start,
                (x_start + 1.) * self.spacing,
                y_start
            );
        }

        for j in 0..Self::NUM_HORIZ_LINES {
            let x_start = (-Self::NUM_VERT_LINES / 2) as f32;
            let y_start = (j - Self::NUM_VERT_LINES / 2) as f32;

            let mat = glam::Mat4::from_scale_rotation_translation(
                Vec3::new(self.spacing, 0., self.spacing),
                Quat::from_axis_angle(Vec3::Y, util::deg2rad(90.)),
                Vec3::new(x_start, 0., y_start),
            );

            mvps.push(mat);
            println!(
                "Vertical line ({}, 0, {}) to ({}, 0, {})",
                x_start,
                y_start * self.spacing,
                x_start,
                (y_start + 1.) * self.spacing,
            );
        }

        mvps
    }
}
