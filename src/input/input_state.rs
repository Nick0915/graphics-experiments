pub struct InputState {
    pub mouse_pos: (f32, f32),
    pub prev_drag_pos: (f32, f32),
    pub drag_amount: (f32, f32),
    pub is_dragging: bool,
    pub vertical_scroll: f32
}

impl InputState {
    pub fn new() -> Self {
        Self {
            mouse_pos: (0., 0.),
            prev_drag_pos: (0., 0.),
            drag_amount: (0., 0.),
            is_dragging: false,
            vertical_scroll: 0.,
        }
    }

    pub fn update(&mut self) {
        self.drag_amount = (0., 0.);
        self.vertical_scroll = 0.;
    //     self.drag_amount = if self.is_dragging {
    //         (
    //             self.mouse_pos.0 - self.prev_mouse_pos.0,
    //             self.mouse_pos.1 - self.prev_mouse_pos.1,
    //         )
    //     } else {
    //         (0., 0.)
    //     };

    //     self.prev_mouse_pos = self.mouse_pos;
    //     self.vertical_scroll = 0.;
    }
}
