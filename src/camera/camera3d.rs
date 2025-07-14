pub struct Camera3D {
    eye: glam::Vec3,
    target: glam::Vec3,
    fov_y: f32,
    aspect: (f32, f32),
    z_clip: (f32, f32),
}

impl Camera3D {
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
            z_clip
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
}
