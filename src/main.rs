#![allow(unused)]

use dotenv;
use gl::{self, types::*};
use glfw::ffi::glfwGetTime;
use log;
use std::{self, thread::current};

use crate::graphics::{shader, Mesh};

mod camera;
mod constants;
mod graphics;
mod input;
mod util;
mod window;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let mut window = window::Window::new();
    let mut input_state = input::InputState::new();
    let mut camera = camera::FocusCamera3D::new(
        (0., 0., 5.),
        (0., 0., 0.),
        util::deg2rad(40.),
        (
            constants::WINDOW_SIZE.0 as f32,
            constants::WINDOW_SIZE.1 as f32,
        ),
        (0.1, 100.),
    );

    #[rustfmt::skip]
    let vertices: Vec<f32> = vec![
        //  x,    y,    z
         -1.0, -1.0,  0.0, // 0: bottom-left
          1.0, -1.0,  0.0, // 1: bottom-right
         -1.0,  1.0,  0.0, // 2: top-left
          1.0,  1.0,  0.0, // 3: top-right
    ];

    // pos, color
    let attr_layout = vec![3];

    #[rustfmt::skip]
    let indices: Vec<u32> = vec![
        1, 2, 0,    // top-left tri
        1, 3, 2     // bottom-right tri
    ];

    let vertex_source = include_str!("../shader/main.vert");
    let fragment_source = include_str!("../shader/main.frag");

    let shader_program = graphics::ShaderProgram::new(vertex_source, fragment_source);
    shader_program.r#use();

    // let mesh = Mesh::from_list(vertices, attr_layout, indices, shader_program);
    let mesh = Mesh::from_basic_obj(include_str!("../models/suzanne.obj"), shader_program);

    unsafe {
        gl::ClearColor(
            constants::CLEAR_COLOR.0,
            constants::CLEAR_COLOR.1,
            constants::CLEAR_COLOR.2,
            constants::CLEAR_COLOR.3,
        );
        gl::Enable(gl::DEPTH_TEST);
    }

    let mut start_time = 0.;
    unsafe {
        start_time = glfwGetTime();
    }
    let mut last_frame_time = start_time;
    let mut delta = 0.;

    while !window.should_close() {
        input_state.update();
        window.process_input(&mut input_state);
        camera.zoom(input_state.vertical_scroll);
        camera.r#move(
            (
                input_state.wasd_vec.0,
                input_state.wasd_vec.1,
                input_state.space_crouch,
            ),
            delta,
        );

        let mut mvp = camera.projection() * camera.view();
        shader_program.uniform_matrix4f("u_MVP", &mvp.to_cols_array()[0]);

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // vertex_array.bind();
            // vertex_buffer.bind();
            // index_buffer.bind();
            // gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            mesh.draw();

            if gl::GetError() != gl::NO_ERROR {
                panic!("There was a GL error!");
            }
        }

        window.draw();

        let mut current_time = 0.;
        unsafe {
            current_time = glfwGetTime();
        }
        delta = (current_time - last_frame_time) as f32;
        last_frame_time = current_time;
    }
}
