use glfw::{self, PWindow};

use crate::input::*;

pub fn mouse_button_callback(
    window_handle: &mut PWindow,
    mouse_button: glfw::MouseButton,
    action: glfw::Action,
    modifiers: glfw::Modifiers,
    input_state: &mut InputState,
) {
    match mouse_button {
        glfw::MouseButton::Left => match action {
            glfw::Action::Press => {
                input_state.is_dragging = true;
                input_state.prev_drag_pos = input_state.mouse_pos;
            }
            glfw::Action::Release => {
                input_state.is_dragging = false;
                input_state.prev_drag_pos = (-1., -1.);
            }
            _ => {}
        },
        glfw::MouseButton::Right => {}
        glfw::MouseButton::Middle => {}
        _ => {}
    }
}

pub fn cursor_pos_callback(
    window_handle: &mut PWindow,
    x: f64,
    y: f64,
    input_state: &mut InputState,
) {
    input_state.mouse_pos.0 = x as f32;
    input_state.mouse_pos.1 = y as f32;

    if input_state.is_dragging {
        // let drag = (
        //     input_state.mouse_pos.0 - input_state.prev_drag_pos.0,
        //     input_state.mouse_pos.1 - input_state.prev_drag_pos.1,
        // );
        input_state.drag_amount.0 += input_state.mouse_pos.0 - input_state.prev_drag_pos.0;
        input_state.drag_amount.1 += input_state.mouse_pos.1 - input_state.prev_drag_pos.1;

        input_state.prev_drag_pos = input_state.mouse_pos;
    }
}

pub fn scroll_callback(window_handle: &mut PWindow, x: f64, y: f64, input_state: &mut InputState) {
    input_state.vertical_scroll = y as f32;
}

pub fn key_callback(
    window_handle: &mut PWindow,
    key: glfw::Key,
    scancode: glfw::Scancode,
    action: glfw::Action,
    modifiers: glfw::Modifiers,
    input_state: &mut InputState,
) {
    match (key, action) {
        (glfw::Key::Escape, glfw::Action::Release) => {
            window_handle.set_should_close(true);
        },
        (glfw::Key::W | glfw::Key::Up, glfw::Action::Press) => {
            input_state.wasd_vec.1 += 1.;
        },
        (glfw::Key::A | glfw::Key::Left, glfw::Action::Press) => {
            input_state.wasd_vec.0 -= 1.;
        },
        (glfw::Key::S | glfw::Key::Down, glfw::Action::Press) => {
            input_state.wasd_vec.1 -= 1.;
        },
        (glfw::Key::D | glfw::Key::Right, glfw::Action::Press) => {
            input_state.wasd_vec.0 += 1.;
        },
        (glfw::Key::W | glfw::Key::Up, glfw::Action::Release) => {
            input_state.wasd_vec.1 -= 1.;
        },
        (glfw::Key::A | glfw::Key::Left, glfw::Action::Release) => {
            input_state.wasd_vec.0 += 1.;
        },
        (glfw::Key::S | glfw::Key::Down, glfw::Action::Release) => {
            input_state.wasd_vec.1 += 1.;
        },
        (glfw::Key::D | glfw::Key::Right, glfw::Action::Release) => {
            input_state.wasd_vec.0 -= 1.;
        },
        (glfw::Key::Space, glfw::Action::Press) => {
            input_state.space_crouch += 1.;
        },
        (glfw::Key::Space, glfw::Action::Release) => {
            input_state.space_crouch -= 1.;
        },
        (glfw::Key::LeftControl, glfw::Action::Press) => {
            input_state.space_crouch -= 1.;
        },
        (glfw::Key::LeftControl, glfw::Action::Release) => {
            input_state.space_crouch += 1.;
        },
        _ => {}
    }
}
