use crate::{camera::{Camera, FocusCamera3D}, graphics::*};

use glam::*;

pub struct Model {
    mesh: Mesh,
    translation: Vec3,
    scale: Vec3,
    rotation: Quat,
}

impl Drawable for Model {
    // draws this model's mesh
    fn draw<T: Camera>(&self, camera: &T) {
        // get model-view-projection matrix from camera, upload it as uniform
        let mut mvp = camera.projection_mat() * camera.view_mat() * self.model_mat();
        self.mesh.uniform_matrix4f("u_MVP", &mvp.to_cols_array()[0]);

        self.mesh.draw();
    }
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
    pub fn model_mat(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }

    /// rotate this model by given axis-angle
    pub fn rotate(&mut self, axis: Vec3, amount: f32) {
        let new_rotation = Quat::from_axis_angle(axis, amount);
        self.rotation *= new_rotation;
    }

    /// translate this model by given amount
    pub fn translate(&mut self, translation: Vec3) {
        self.translation += translation;
    }
}
