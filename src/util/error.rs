/// handles GLFW errors by printing and panicking
pub fn error_callback(error: glfw::Error, description: String) {
    log::error!("{:?}: {:?}", error, description);
    panic!("Exiting due to error");
}