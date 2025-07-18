#![allow(unused)]

use dotenv;
use gl::{self, types::*};
use glfw::ffi::glfwGetTime;
use log;
use std::{self, thread::current, io::Write};

use crate::{graphics::*, model::Model};

mod camera;
mod constants;
mod graphics;
mod input;
mod util;
mod window;

fn main() {
    // initialize logging
    dotenv::dotenv().ok();
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            writeln!(
                buf,
                // [file:line (time) level]> message
                "[{}:{} ({}) {}]> {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%H:%M:%S"),
                record.level(),
                record.args()
            )
        }).filter(Some("logger_example"), log::LevelFilter::Debug)
        .init();

    log::info!("hi");

    let mut window = window::Window::new();
    let mut input_state = input::InputState::new();
    let mut camera = camera::FocusCamera3D::new(
        (0., 3., 10.),   // camera pos
        (0., 1.5, 0.),   // lookAt pos
        util::deg2rad(50.), // vertical fov
        (
            constants::WINDOW_SIZE.0 as f32,
            constants::WINDOW_SIZE.1 as f32,
        ),  // aspect, used to calculate aspect ratio
        (0.1, 100.), // near and far clip
    );

    // compile and link shaders used
    let vertex_source = include_str!("../shader/main.vert");
    let fragment_source = include_str!("../shader/main.frag");
    let shader_program = graphics::ShaderProgram::new(vertex_source, fragment_source);

    // create mesh
    let mut model_pre = 0.;
    let mut model_post = 0.;
    unsafe { model_pre = glfwGetTime(); }
    let mesh = Mesh::from_obj(include_str!("../models/suzanne_nontrivial.obj"), shader_program);
    unsafe { model_post = glfwGetTime(); }
    log::info!("took {:.2} ms to load", (model_post - model_pre) * 1000.);

    let mut suzanne = Model::from_default_transform(mesh);
    suzanne.translate(glam::Vec3::new(0., 1.5, 0.));

    // create grid
    let mut grid = SquareGrid::new(25, 1., 0.);

    // set clear (background) color and enable depth testing
    unsafe {
        gl::ClearColor(
            constants::CLEAR_COLOR.0,
            constants::CLEAR_COLOR.1,
            constants::CLEAR_COLOR.2,
            constants::CLEAR_COLOR.3,
        );
        gl::Enable(gl::DEPTH_TEST);
    }

    // timing stuff
    let mut start_time = 0.;
    unsafe {
        start_time = glfwGetTime();
    }
    let mut last_frame_time = start_time;
    let mut delta = 0.;

    // fps stuff
    let framerate_update_interval = 0.50;
    let mut framerate_update_timer = framerate_update_interval;
    let mut num_frames_in_interval = 0;

    // main loop
    while !window.should_close() {
        // reset input state
        input_state.reset();

        // collect inputs
        window.process_input(&mut input_state);

        // use inputs to update camera
        camera.update_aspect(input_state.aspect);
        camera.zoom(input_state.vertical_scroll);
        camera.r#move(
            (
                input_state.wasd_vec.0,
                input_state.wasd_vec.1,
                input_state.space_crouch,
            ),
            delta,
        );

        // update model rotation
        // suzanne.rotate(glam::Vec3::Y, util::deg2rad(15.) * delta);

        unsafe {
            // clear last frame
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // draw grid
            grid.draw(&camera);

            // draw model
            suzanne.draw(&camera);

            // lines_vertex_array.bind();
            // lines_index_buffer.bind();
            // lines_vertex_buffer.bind();
            // gl::DrawElements(gl::LINES, 4, gl::UNSIGNED_INT, std::ptr::null());
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // present the newly drawn frame
        window.present_frame();

        // update timing stuff
        let mut current_time = 0.;
        unsafe {
            current_time = glfwGetTime();
        }
        delta = (current_time - last_frame_time) as f32;
        last_frame_time = current_time;

        // update fps stuff
        num_frames_in_interval += 1;
        framerate_update_timer -= delta;
        if framerate_update_timer <= 0. {
            let duration = framerate_update_interval - framerate_update_timer;
            let fps = num_frames_in_interval as f32 / duration;
            window.window_handle.set_title(format!("{} | avg fps: {:.2}", constants::WINDOW_TITLE, fps).as_str());

            num_frames_in_interval = 0;
            framerate_update_timer = framerate_update_interval;
        }
    }
}
