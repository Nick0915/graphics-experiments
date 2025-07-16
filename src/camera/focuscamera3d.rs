pub struct FocusCamera3D {
    eye: glam::Vec3,
    target: glam::Vec3,
    fov_y: f32,
    aspect: (f32, f32),
    z_clip: (f32, f32),
}

use log::info;

use crate::*;

impl FocusCamera3D {
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

    pub fn view(&self) -> glam::Mat4 {
        glam::Mat4::look_at_rh(self.eye, self.target, glam::Vec3::Y)
    }

    pub fn projection(&self) -> glam::Mat4 {
        glam::Mat4::perspective_rh_gl(
            self.fov_y,
            self.aspect.0 / self.aspect.1,
            self.z_clip.0,
            self.z_clip.1,
        )
    }

    pub fn zoom(&mut self, amt: f32) {
        self.fov_y -= util::deg2rad(constants::ZOOM_AMT * amt);
        self.fov_y = self.fov_y.clamp(util::deg2rad(10.), util::deg2rad(90.));
    }

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
        self.eye.y = self.eye.y.clamp(MIN_HEIGHT, MAX_HEIGHT);

        // third: change angle around focus
        let rotation =
            glam::Quat::from_axis_angle(glam::Vec3::Y, x_axis * constants::REVOLVE_SPEED * delta);

        let new_offset = rotation * (self.eye - self.target);
        let new_eye = self.target + new_offset;
        self.eye = new_eye;
    }
}
