use std::{collections::HashMap, ffi::CStr, os::raw::c_void};

use gl::types::*;

/// handles GLFW errors by printing and panicking
pub fn error_callback(error: glfw::Error, description: String) {
    log::error!("{:?}: {:?}", error, description);
    panic!("Exiting due to error");
}

pub extern "system" fn gl_error_callback(
    source: GLenum,
    r#type: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const i8,
    user_param: *mut c_void,
) {
    let types: HashMap<GLenum, &str> = HashMap::from([
        (gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR, "depreciated behavior"),
        (gl::DEBUG_TYPE_ERROR, "error"),
        (gl::DEBUG_TYPE_MARKER, "marker"),
        (gl::DEBUG_TYPE_OTHER, "other"),
        (gl::DEBUG_TYPE_PERFORMANCE, "performance"),
        (gl::DEBUG_TYPE_POP_GROUP, "pop group"),
        (gl::DEBUG_TYPE_PORTABILITY, "portability"),
        (gl::DEBUG_TYPE_PUSH_GROUP, "push group"),
        (gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR, "undefined behavior"),
    ]);

    let severities: HashMap<GLenum, &str> = HashMap::from([
        (gl::DEBUG_SEVERITY_HIGH, "high"),
        (gl::DEBUG_SEVERITY_LOW, "low"),
        (gl::DEBUG_SEVERITY_MEDIUM, "medium"),
        (gl::DEBUG_SEVERITY_NOTIFICATION, "notification"),
    ]);

    let first_type = gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR;
    let first_severity = gl::DEBUG_SEVERITY_HIGH;

    let severity_msg = severities[&severity];
    let type_msg = types[&r#type];

    match severity {
        gl::DEBUG_SEVERITY_NOTIFICATION => {
            // unsafe {
            //     log::info!("GL notification ({} type): {}", type_msg, CStr::from_ptr(message).to_str().unwrap());
            // }
        }
        _ => unsafe {
            log::error!(
                "** GL ** ({} type, {} severity): {}",
                type_msg,
                severity_msg,
                CStr::from_ptr(message).to_str().unwrap()
            );
        },
    }
}
