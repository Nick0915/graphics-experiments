use crate::graphics::*;

use glam::*;

pub struct Model {
    mesh: Mesh,
    translation: Vec3,
    scale: Vec3,
    rotation: Quat,
}

impl Model {
    pub fn from_default_transform(mesh: Mesh) -> Self {
        Self {
            mesh,
            translation: Vec3::ZERO,
            scale: Vec3::ONE,
            rotation: Quat::from_euler(EulerRot::XYZ, 0., 0., 0.),
        }
    }

    /// gets the model matrix of this model
    pub fn model(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }

    // rotate this model by given axis-angle
    pub fn rotate(&mut self, axis: Vec3, amount: f32) {
        let new_rotation = Quat::from_axis_angle(axis, amount);
        self.rotation *= new_rotation;
    }

    // draws this model's mesh
    pub fn draw(&self) {
        self.mesh.draw();
    }
}
