use glfw::{self, PWindow};

use crate::window::Window;

pub fn mouse_button_callback(
    window_handle: &mut PWindow,
    mouse_button: glfw::MouseButton,
    action: glfw::Action,
    modifiers: glfw::Modifiers,
) {
}

pub fn cursor_pos_callback(window_handle: &mut PWindow, x: f64, y: f64) {}

pub fn scroll_callback(window_handle: &mut PWindow, x: f64, y: f64) {}

pub fn key_callback(
    window_handle: &mut PWindow,
    key: glfw::Key,
    scancode: glfw::Scancode,
    action: glfw::Action,
    modifiers: glfw::Modifiers,
) {
    match key {
        glfw::Key::Escape => {
            // window.close()
            window_handle.set_should_close(true);
        },
        _ => {},
    }
}
