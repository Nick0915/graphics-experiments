pub const WINDOW_SIZE: (u32, u32) = (1000, 1000);
pub const WINDOW_TITLE: &str = "Hello world!";

pub const CLEAR_COLOR: (f32, f32, f32, f32) = (0.15, 0.15, 0.22, 0.0);

/// how fast scrolling should change the FOV of the camera
pub const ZOOM_AMT: f32 = 5.;

/// how fast W, S, Space, and LeftControl should move the camera
pub const MOVE_SPEED: f32 = 20.;

/// how fast A and D should make the camera revolve around the center
pub const REVOLVE_SPEED: f32 = 3.5;
