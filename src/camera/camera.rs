pub trait Camera {
    fn view_mat(&self) -> glam::Mat4;
    fn projection_mat(&self) -> glam::Mat4;
}