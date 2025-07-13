pub fn error_callback(error: glfw::Error, description: String) {
    log::error!("{:?}: {:?}", error, description);
}