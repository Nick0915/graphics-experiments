use std::ffi::CString;

use gl::types::*;

pub struct ShaderProgram {
    id: u32,
}

impl ShaderProgram {
    pub fn new(vertex_src: &str, fragment_src: &str) -> Self {
        let mut prog_id = 0;
        let mut vert_id = 0;
        let mut frag_id = 0;

        let c_vertex_src = CString::new(vertex_src.as_bytes()).unwrap();
        let c_fragment_src = CString::new(fragment_src.as_bytes()).unwrap();

        let mut success = 0;

        let mut failed = false;

        unsafe {
            vert_id = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vert_id, 1, &c_vertex_src.as_ptr(), std::ptr::null());
            gl::CompileShader(vert_id);
            gl::GetShaderiv(vert_id, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut log_len = 0;
                gl::GetShaderiv(vert_id, gl::INFO_LOG_LENGTH, &mut log_len);

                let mut info_log = Vec::with_capacity(log_len as usize);
                info_log.set_len((log_len as usize) - 1);
                let mut actual_len = 0;

                gl::GetShaderInfoLog(vert_id, log_len, &mut actual_len, info_log.as_mut_ptr());
                log::error!(
                    "Vertex shader failed to compile! Log: \n{}",
                    String::from_utf8(std::mem::transmute(info_log)).unwrap()
                );

                failed = true;
            }

            frag_id = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(frag_id, 1, &c_fragment_src.as_ptr(), std::ptr::null());
            gl::CompileShader(frag_id);
            gl::GetShaderiv(frag_id, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut log_len = 0;
                gl::GetShaderiv(frag_id, gl::INFO_LOG_LENGTH, &mut log_len);

                let mut info_log = Vec::with_capacity(log_len as usize);
                info_log.set_len((log_len as usize) - 1);
                let mut actual_len = 0;

                gl::GetShaderInfoLog(frag_id, log_len, &mut actual_len, info_log.as_mut_ptr());
                log::error!(
                    "Fragment shader failed to compile! Log: \n{}",
                    String::from_utf8(std::mem::transmute(info_log)).unwrap()
                );

                failed = true;
            }

            prog_id = gl::CreateProgram();
            gl::AttachShader(prog_id, vert_id);
            gl::AttachShader(prog_id, frag_id);
            gl::LinkProgram(prog_id);
            gl::GetProgramiv(prog_id, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut log_len = 0;
                gl::GetProgramiv(prog_id, gl::INFO_LOG_LENGTH, &mut log_len);

                let mut info_log = Vec::with_capacity(log_len as usize);
                info_log.set_len((log_len as usize) - 1);
                let mut actual_len = 0;

                gl::GetProgramInfoLog(prog_id, log_len, &mut actual_len, info_log.as_mut_ptr());
                log::error!(
                    "Shader program failed to link! Log: \n{}",
                    String::from_utf8(std::mem::transmute(info_log)).unwrap()
                );

                failed = true;
            }

            gl::DeleteShader(vert_id);
            gl::DeleteShader(frag_id);

            if failed {
                panic!("Shader program creation failed!");
            }
        }

        Self { id: prog_id }
    }
}
