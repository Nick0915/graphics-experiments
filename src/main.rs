#![allow(unused)]

use gl::{self, types::*};
use log;
use dotenv;
use std;

mod util;
mod graphics;
mod window;
mod camera;
mod input;
mod constants;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let mut window = window::Window::new();
    let mut input_state = input::InputState::new();
    // let mut camera = camera::Camera2D::new();
    let mut camera = camera::Camera3D::new(
        (5., 0., -5.),
        (0., 0., 0.),
        0.785398,
        (constants::WINDOW_SIZE.0 as f32, constants::WINDOW_SIZE.1 as f32),
        (0.1, 1000.)
    );

    // let mut vertex_buffer = graphics::BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    // vertex_buffer.bind();
    // vertex_buffer.buffer_data(&vertices);

    // let mut element_buffer = graphics::BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    // element_buffer.bind();
    // element_buffer.buffer_data(&indices);

    // let mut vertex_array = graphics::VAO::new(vertex_layout, gl::FLOAT, &vertices);
    // vertex_array.bind();

    let vertex_source = include_str!("../shader/main.vert");
    let fragment_source = include_str!("../shader/main.frag");

    let program = graphics::ShaderProgram::new(
        vertex_source, fragment_source
    );

    let cube_mesh = graphics::Mesh::from_obj(include_str!("../models/cube.obj"), program);

    unsafe {
        gl::ClearColor(
            constants::CLEAR_COLOR.0,
            constants::CLEAR_COLOR.1,
            constants::CLEAR_COLOR.2,
            constants::CLEAR_COLOR.3,
        );
    }

    let mut u_loc_resolution = 0;
    let mut u_loc_zoom = 0;
    let mut u_loc_pan = 0;

    while !window.should_close() {
        input_state.update();
        window.process_input(&mut input_state);
        // camera.pan(input_state.drag_amount);
        // camera.zoom(input_state.vertical_scroll);

        let mvp = camera.view().mul_mat4(&camera.projection());
        program.uniform_matrix4f("u_MVP", &mvp.to_cols_array()[0]);

        unsafe {
            // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            cube_mesh.draw();

            // vertex_array.bind();
            // vertex_buffer.bind();
            // element_buffer.bind();
            // gl::DrawElements(gl::TRIANGLE_STRIP, 4, gl::UNSIGNED_BYTE, std::mem::transmute(0i64));
        }

        window.draw();
    }

    // vertex_array.free();
    // vertex_buffer.free();
    // element_buffer.free();
}
