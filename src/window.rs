use gl;
use glfw::{self, Context, WindowEvent};

use crate::*;

pub struct Window {
    glfw: glfw::Glfw,
    window_handle: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    width: u32,
    height: u32,
    title: String,
}

impl Window {
    pub fn new() -> Window {
        let mut glfw = glfw::init(util::error::error_callback).expect("Couldn't initialize GLFW!");

        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        let (mut window, events) = glfw
            .create_window(
                constants::WINDOW_SIZE.0,
                constants::WINDOW_SIZE.1,
                constants::WINDOW_TITLE,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create window!");

        window.make_current();
        window.set_all_polling(true);

        gl::load_with(|s| glfw.get_proc_address_raw(s));
        unsafe {
            gl::Viewport(
                0,
                0,
                constants::WINDOW_SIZE.0 as i32,
                constants::WINDOW_SIZE.1 as i32,
            );
        }
        // glfw.set_swap_interval(glfw::SwapInterval::None);
        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

        Window {
            glfw,
            window_handle: window,
            events,
            width: constants::WINDOW_SIZE.0,
            height: constants::WINDOW_SIZE.1,
            title: constants::WINDOW_TITLE.to_string(),
        }
    }

    pub fn close(&mut self) {
        self.window_handle.set_should_close(true);
    }

    pub fn should_close(&self) -> bool {
        self.window_handle.should_close()
    }

    pub fn process_events(&mut self, input_state: &mut input::InputState) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Close => self.window_handle.set_should_close(true),
                WindowEvent::FramebufferSize(new_width, new_height) => {
                    self.width = new_width as u32;
                    self.height = new_height as u32;

                    unsafe {
                        gl::Viewport(0, 0, new_width, new_height);
                    }
                }
                WindowEvent::MouseButton(mouse_button, action, modifiers) => {
                    input::mouse_button_callback(&mut self.window_handle, mouse_button, action, modifiers, input_state);
                }
                WindowEvent::CursorPos(x, y) => input::cursor_pos_callback(&mut self.window_handle, x, y, input_state),
                WindowEvent::Scroll(x, y) => input::scroll_callback(&mut self.window_handle, x, y, input_state),
                WindowEvent::Key(key, scancode, action, modifiers) => {
                    input::key_callback(&mut self.window_handle, key, scancode, action, modifiers, input_state);
                }
                _ => {}
            }
        }
    }

    pub fn process_input(&mut self, input_state: &mut input::InputState) {
        self.glfw.poll_events();
        self.process_events(input_state);
    }

    pub fn draw(&mut self) {
        self.window_handle.swap_buffers();
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn title(&self) -> String {
        self.title.to_owned()
    }
}
