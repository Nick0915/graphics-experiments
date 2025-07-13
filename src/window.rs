use gl;
use glfw::{self, Context, WindowEvent};

use crate::util;

pub struct Window {
    glfw: glfw::Glfw,
    window_handle: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    width: u32,
    height: u32,
    title: String,
}

impl Window {
    pub fn init() -> Window {
        let mut glfw = glfw::init(util::error::error_callback).expect("Couldn't initialize GLFW!");

        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        let (mut window, events) = glfw
            .create_window(
                util::constants::WINDOW_SIZE.0,
                util::constants::WINDOW_SIZE.1,
                util::constants::WINDOW_TITLE,
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
                util::constants::WINDOW_SIZE.0 as i32,
                util::constants::WINDOW_SIZE.1 as i32,
            );
        }
        glfw.set_swap_interval(glfw::SwapInterval::None);

        Window {
            glfw,
            window_handle: window,
            events,
            width: util::constants::WINDOW_SIZE.0,
            height: util::constants::WINDOW_SIZE.1,
            title: util::constants::WINDOW_TITLE.to_string(),
        }
    }

    pub fn close(&mut self) {
        self.window_handle.set_should_close(true);
    }

    pub fn should_close(&self) -> bool {
        self.window_handle.should_close()
    }

    pub fn process_events(&mut self) {
        use util::callback::*;
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
                    mouse_button_callback(&mut self.window_handle, mouse_button, action, modifiers);
                }
                WindowEvent::CursorPos(x, y) => cursor_pos_callback(&mut self.window_handle, x, y),
                WindowEvent::Scroll(x, y) => scroll_callback(&mut self.window_handle, x, y),
                WindowEvent::Key(key, scancode, action, modifiers) => {
                    key_callback(&mut self.window_handle, key, scancode, action, modifiers);
                }
                _ => {}
            }
        }
    }

    pub fn update(&mut self) {
        self.glfw.poll_events();
        self.process_events();
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
