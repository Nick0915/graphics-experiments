use crate::camera::Camera;

pub trait Drawable {
    fn draw<T: Camera>(&mut self, camera: &T);
}