#![allow(unused)]

use dotenv;
use gl::{self, types::*};
use log;
use std;

use crate::graphics::shader;

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
    // let mut camera = camera::Camera2D::new();
    // let mut camera = camera::Camera3D::new(
    //     (5., 0., -5.),
    //     (0., 0., 0.),
    //     0.785398,
    //     (constants::WINDOW_SIZE.0 as f32, constants::WINDOW_SIZE.1 as f32),
    //     (0.1, 1000.)
    // );

   #[rustfmt::skip]
    let vertices: Vec<f32> = vec![
         -0.5,  0.5, // 0: top-left pos
         -0.5, -0.5, // 1: bottom-left pos
          0.5, -0.5, // 2: bottom-right pos
          0.5,  0.5, // 3: top-right por
    ];

    #[rustfmt::skip]
    let indices: Vec<u32> = vec![
        0, 1, 3,    // bottom-left tri
        3, 1, 2     // top-left tri
    ];

    let vertex_source = include_str!("../shader/main.vert");
    let fragment_source = include_str!("../shader/main.frag");

    let shader_program = graphics::ShaderProgram::new(vertex_source, fragment_source);
    shader_program.r#use();

    let mut index_buffer = graphics::BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    index_buffer.bind();
    index_buffer.buffer_data(&indices);

    let mut vertex_buffer = graphics::BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vertex_buffer.bind();
    vertex_buffer.buffer_data(&vertices);

    let attr_layout = vec![2];
    let mut vertex_array = graphics::VAO::new(attr_layout, gl::FLOAT, &vertices);
    vertex_array.bind();

    unsafe {
        if gl::GetError() != gl::NO_ERROR {
            panic!("There was a GL error!");
        }
    }

    // let mut vbo = 0u32;
    // let mut vao = 0u32;
    // let mut ebo = 0u32;
    // unsafe {
    //     gl::GenVertexArrays(1, &mut vao);
    //     gl::BindVertexArray(vao);

    //     gl::GenBuffers(1, &mut ebo);
    //     gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    //     gl::BufferData(
    //         gl::ELEMENT_ARRAY_BUFFER,
    //         (std::mem::size_of::<u32>() * indices.len()) as isize,
    //         std::mem::transmute(&indices[0]),
    //         gl::STATIC_DRAW
    //     );

    //     gl::GenBuffers(1, &mut vbo);
    //     gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    //     gl::BufferData(
    //         gl::ARRAY_BUFFER,
    //         (std::mem::size_of::<f32>() * vertices.len()) as isize,
    //         std::mem::transmute(&vertices[0]),
    //         gl::STATIC_DRAW,
    //     );

    //     gl::VertexAttribPointer(
    //         0,
    //         2,
    //         gl::FLOAT,
    //         gl::FALSE,
    //         (2 * std::mem::size_of::<f32>()) as i32,
    //         std::mem::transmute(&vertices[0])
    //     );
    //     gl::EnableVertexAttribArray(0);
    // }

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

        // let mvp = camera.view().mul_mat4(&camera.projection());
        // shader_program.uniform_matrix4f("u_MVP", &mvp.to_cols_array()[0]);

        unsafe {
            // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            vertex_array.bind();
            vertex_buffer.bind();
            index_buffer.bind();
            gl::DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }

        window.draw();
    }
}
