#![allow(unused)]

use gl::{self, types::*};
use log;
use dotenv;
use std;

use crate::{graphics::*, util::constants};

mod util;
mod graphics;
mod window;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let mut window = window::Window::init();

    #[rustfmt::skip]
    let vertices: [f32; 8] = [
        -1.0,  1.0, // 0: top-left pos, uv
        -1.0, -1.0, // 1: bottom-left pos, uv
         1.0, -1.0, // 2: bottom-right pos, uv
         1.0,  1.0, // 3: top-right pos, uv
    ];

    // 2 floats for position (x, y) then two floats for uv (s, t)
    let vertex_layout = vec![2];

    #[rustfmt::skip]
    let indices: [u8; 4] = [
        1, 3, 0,    // bottom-left tri
        3,          // top-left tri
    ];

    let mut vertex_buffer = buffer::BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vertex_buffer.bind();
    vertex_buffer.buffer_data(&vertices);

    let mut element_buffer = buffer::BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    element_buffer.bind();
    element_buffer.buffer_data(&indices);

    let mut vertex_array = vao::VAO::new(vertex_layout, gl::FLOAT, &vertices);
    vertex_array.bind();

    let vertex_source = include_str!("../shader/main.vert");
    let fragment_source = include_str!("../shader/main.frag");

    let program = shader::ShaderProgram::new(
        vertex_source, fragment_source
    );
    program.r#use();

    unsafe {
        gl::ClearColor(
            constants::CLEAR_COLOR.0,
            constants::CLEAR_COLOR.1,
            constants::CLEAR_COLOR.2,
            constants::CLEAR_COLOR.3,
        );
    }

    while !window.should_close() {
        window.process_input();

        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            vertex_array.bind();
            vertex_buffer.bind();
            element_buffer.bind();
            gl::DrawElements(gl::TRIANGLE_STRIP, 4, gl::UNSIGNED_BYTE, std::mem::transmute(0i64));
        }

        window.draw();
    }

    vertex_array.free();
    vertex_buffer.free();
    element_buffer.free();
}
