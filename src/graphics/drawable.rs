use crate::camera::Camera;

pub trait Drawable {
    fn draw<T: Camera>(&self, camera: &T);
}