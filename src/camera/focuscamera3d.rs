/// represents a 3D camera which focuses on a central object (target) and swivels
/// around it
///
/// it can also raise and lower vertically, and move away and toward the
/// target horizontally
pub struct FocusCamera3D {
    eye: glam::Vec3,
    target: glam::Vec3,
    fov_y: f32,
    aspect: (f32, f32),
    z_clip: (f32, f32),
}

use log::info;

use crate::{camera::Camera, *};

impl Camera for FocusCamera3D {
    /// gets the view matrix of the camera
    fn view_mat(&self) -> glam::Mat4 {
        glam::Mat4::look_at_rh(self.eye, self.target, glam::Vec3::Y)
    }

    /// gets the projection matrix of the camera
    fn projection_mat(&self) -> glam::Mat4 {
        glam::Mat4::perspective_rh_gl(
            self.fov_y,
            self.aspect.0 / self.aspect.1,
            self.z_clip.0,
            self.z_clip.1,
        )
    }

}

impl FocusCamera3D {
    /// creates a camera
    pub fn new(
        eye: (f32, f32, f32),
        target: (f32, f32, f32),
        fov_y: f32,
        aspect: (f32, f32),
        z_clip: (f32, f32),
    ) -> Self {
        Self {
            eye: glam::Vec3::new(eye.0, eye.1, eye.2),
            target: glam::Vec3::new(target.0, target.1, target.2),
            fov_y,
            aspect,
            z_clip,
        }
    }

    /// zooms the camera by the given amount
    pub fn zoom(&mut self, amt: f32) {
        // not multiplied by delta time because zooming is a discrete "teleport"-
        // like action done with the scroll wheel, not a continuous motion
        self.fov_y -= util::deg2rad(constants::ZOOM_AMT * amt);
        self.fov_y = self.fov_y.clamp(util::deg2rad(10.), util::deg2rad(90.));
    }

    /// moves the camera given movement axes
    ///
    /// the y and z inputs move the camera as expected in its local y and z axes,
    /// but the x input revolves the camera around the target
    pub fn r#move(&mut self, (x_axis, y_axis, z_axis): (f32, f32, f32), delta: f32) {
        // first: change distance away from focus
        let (direction, mut cur_dist) = (self.eye - self.target).normalize_and_length();

        // make sure we don't move too far from the focus
        let mut new_dist = cur_dist - y_axis * constants::MOVE_SPEED * delta;
        new_dist = new_dist.clamp(self.z_clip.0, self.z_clip.1);

        let new_offset = direction * new_dist;

        let new_eye = self.target + new_offset;
        self.eye = new_eye;

        // second: change height from ground
        const MAX_HEIGHT: f32 = 12.5;
        const MIN_HEIGHT: f32 = -12.5;

        let translation = z_axis * constants::MOVE_SPEED * delta;
        self.eye += glam::Vec3::Y * translation;

        // make sure we don't move too far up or down
        self.eye.y = self.eye.y.clamp(MIN_HEIGHT, MAX_HEIGHT);

        // third: change angle around focus
        let rotation =
            glam::Quat::from_axis_angle(glam::Vec3::Y, x_axis * constants::REVOLVE_SPEED * delta);

        // rotate self about the target (the rotation axis is the Y axis)
        let new_offset = rotation * (self.eye - self.target);
        let new_eye = self.target + new_offset;
        self.eye = new_eye;
    }
}
