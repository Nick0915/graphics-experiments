use glam;

use crate::*;

pub struct Camera2D {
    pub position: (f32, f32),
    pub zoom: f32,
}

impl Camera2D {
    pub fn new() -> Self {
        Self {
            position: (0., 0.),
            zoom: 1.
        }
    }

    pub fn zoom_in(&mut self) {
        self.zoom *= constants::ZOOM_AMT;
    }

    pub fn zoom_out(&mut self) {
        self.zoom /= constants::ZOOM_AMT;
    }

    pub fn zoom(&mut self, direction: f32) {
        self.zoom *= constants::ZOOM_AMT.powf(direction);
    }

    pub fn pan(&mut self, offset: (f32, f32)) {
        self.position.0 -= offset.0 * self.zoom;
        self.position.1 += offset.1 * self.zoom;
    }
}
