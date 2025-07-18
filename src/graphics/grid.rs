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
    model_mats_updated: bool,
}

impl Drawable for Grid10x10 {
    fn draw<T: Camera>(&mut self, camera: &T) {
        let proj_view = camera.projection_mat() * camera.view_mat();

        let num_insts = (Self::NUM_HORIZ_LINES + Self::NUM_VERT_LINES) as usize;

        if !self.model_mats_updated {
            let models = self.generate_model_matrices();
            for inst_id in 0..num_insts {
                let model = models[inst_id as usize % models.len()];

                self.shader_program
                    .uniform_matrix4f(format!("u_model[{}]", inst_id).as_str(), &model.to_cols_array()[0]);
            }

            self.model_mats_updated = true;
        }

        let proj_view = camera.projection_mat() * camera.view_mat();
        self.shader_program.uniform_matrix4f("u_proj_view", &proj_view.to_cols_array()[0]);

        // bind all appropriate buffers/arrays/programs
        self.vertex_array.bind();
        self.vertex_buffer.bind();
        self.shader_program.r#use();

        // todo!()
        unsafe {
            gl::DrawArraysInstanced(gl::LINES, 0, 2, num_insts as i32);
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
                include_str!("../../shader/grid10x10.vert"),
                include_str!("../../shader/main.frag"),
            ),
            vertex_array,
            vertex_buffer,
            model_mats_updated: false,
        }
    }

    const NUM_HORIZ_LINES: i32 = 11;
    const NUM_VERT_LINES: i32 = 11;

    pub fn generate_model_matrices(&self) -> Vec<glam::Mat4> {
        let mut mats: Vec<glam::Mat4> = Vec::new();

        let width = (Self::NUM_VERT_LINES - 1) as f32 * self.spacing;
        let height = (Self::NUM_HORIZ_LINES - 1) as f32 * self.spacing;

        // horizontal lines
        for j in 0..(Self::NUM_HORIZ_LINES) {
            let t = j as f32 / (Self::NUM_HORIZ_LINES - 1) as f32;

            let x_start = - width / 2.;
            let x_end = width / 2.;
            let y = f32::lerp(-height / 2., height / 2., t);

            let line_size = x_end - x_start;

            let horiz_mat = glam::Mat4::from_scale_rotation_translation(
                Vec3::new(line_size, 0., 1.),
                Quat::IDENTITY,
                Vec3::new(x_start, 0., y),
            );
            let vert_mat = glam::Mat4::from_scale_rotation_translation(
                Vec3::new(line_size, 0., 1.),
                Quat::from_axis_angle(Vec3::Y, util::deg2rad(90.)),
                Vec3::new(y, 0., x_start + height),
            );

            mats.push(horiz_mat);
            mats.push(vert_mat);
        }

        mats
    }
}
