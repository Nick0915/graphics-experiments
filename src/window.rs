use gl;
use glfw::{self, Context, WindowEvent};

use crate::{util::logging, *};

/// wrapper around useful window-related objects and values
pub struct Window {
    glfw: glfw::Glfw,
    pub window_handle: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    width: u32,
    height: u32,
    title: String,
}

impl Window {
    /// create a new window
    pub fn new() -> Window {
        // initialize GLFW
        let mut glfw = glfw::init(util::logging::error_callback).expect("Couldn't initialize GLFW!");

        // hint about OpenGL version
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        // create window
        let (mut window, events) = glfw
            .create_window(
                constants::WINDOW_SIZE.0,
                constants::WINDOW_SIZE.1,
                constants::WINDOW_TITLE,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create window!");

        // make all future OpenGL function calls refer to this window's context
        window.make_current();
        // ! this is needed to make sure all inputs works
        // changing this might break input processing code
        window.set_all_polling(true);

        // load OpenGL functions using this window's process address
        gl::load_with(|s| glfw.get_proc_address_raw(s));
        unsafe {
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::DebugMessageCallback(Some(logging::gl_error_callback), std::ptr::null());

            // where to set the bottom-left of the viewport and how big it should be
            // set it at corner of the window and make it the size of the window
            gl::Viewport(
                0,
                0,
                constants::WINDOW_SIZE.0 as i32,
                constants::WINDOW_SIZE.1 as i32,
            );
        }


        // buffering (change to SwapInterval::Sync(1) for double-buffered VSync)
        glfw.set_swap_interval(glfw::SwapInterval::None);

        Window {
            glfw,
            window_handle: window,
            events,
            width: constants::WINDOW_SIZE.0,
            height: constants::WINDOW_SIZE.1,
            title: constants::WINDOW_TITLE.to_string(),
        }
    }

    /// check whether the window is closed
    #[inline]
    pub fn should_close(&self) -> bool {
        self.window_handle.should_close()
    }

    /// process input events queued in the Receiver
    pub fn process_events(&mut self, input_state: &mut input::InputState) {
        // loop over all events
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                // window closed
                WindowEvent::Close => self.window_handle.set_should_close(true),

                // window resized
                WindowEvent::FramebufferSize(new_width, new_height) => {
                    self.width = new_width as u32;
                    self.height = new_height as u32;

                    unsafe {
                        gl::Viewport(0, 0, new_width, new_height);
                    }
                }

                // mouse buttons
                WindowEvent::MouseButton(mouse_button, action, modifiers) => {
                    input::mouse_button_callback(&mut self.window_handle, mouse_button, action, modifiers, input_state);
                }

                // cursor motion
                WindowEvent::CursorPos(x, y) => input::cursor_pos_callback(&mut self.window_handle, x, y, input_state),

                // cursor scroll
                WindowEvent::Scroll(x, y) => input::scroll_callback(&mut self.window_handle, x, y, input_state),

                // keyboard input
                WindowEvent::Key(key, scancode, action, modifiers) => {
                    input::key_callback(&mut self.window_handle, key, scancode, action, modifiers, input_state);
                }
                _ => {}
            }
        }
    }

    /// process all inputs and update the input state accordingly
    pub fn process_input(&mut self, input_state: &mut input::InputState) {
        // need to poll events in order to be able to be able to loop over them in the receiver
        self.glfw.poll_events();
        self.process_events(input_state);
    }

    pub fn present_frame(&mut self) {
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
