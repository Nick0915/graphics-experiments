use crate::constants;

/// contains all relevant input actions that happened this frame
pub struct InputState {
    pub mouse_pos: (f32, f32),
    pub prev_drag_pos: (f32, f32),
    pub drag_amount: (f32, f32),
    pub is_dragging: bool,
    pub vertical_scroll: f32,
    pub wasd_vec: (f32, f32),
    pub space_crouch: f32,
    pub aspect: (u32, u32)
}

impl InputState {
    pub fn new() -> Self {
        Self {
            mouse_pos: (0., 0.),
            prev_drag_pos: (0., 0.),
            drag_amount: (0., 0.),
            is_dragging: false,
            vertical_scroll: 0.,
            wasd_vec: (0., 0.),
            space_crouch: 0.,
            aspect: constants::WINDOW_SIZE,
        }
    }

    /// resets the input state to ready it for the next frame
    pub fn reset(&mut self) {
        self.drag_amount = (0., 0.);
        self.vertical_scroll = 0.;
    }
}
