use std::ffi::CString;

use gl::types::*;


/// wrapper for an OpenGL shader program
#[derive(Copy, Clone)] // since it really only is a u32, copy and clone won't be expensive
pub struct ShaderProgram {
    id: u32,
}

impl ShaderProgram {
    /// compile and link a new shader program given the vertex and fragment shader source code
    pub fn new(vertex_src: &str, fragment_src: &str) -> Self {
        let mut prog_id = 0;
        let mut vert_id = 0;
        let mut frag_id = 0;

        // need C-strings cuz that's what OpenGL expects
        let c_vertex_src = CString::new(vertex_src.as_bytes()).unwrap();
        let c_fragment_src = CString::new(fragment_src.as_bytes()).unwrap();

        let mut success = 0;
        let mut failed = false;

        unsafe {
            // compile the vertex shader
            vert_id = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vert_id, 1, &c_vertex_src.as_ptr(), std::ptr::null());
            gl::CompileShader(vert_id);

            // check for errors
            gl::GetShaderiv(vert_id, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                // get the error string back from the GPU to print it out
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

                // don't end just yet, in case anything else fails too
                failed = true;
            }


            // compile the fragment shader
            frag_id = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(frag_id, 1, &c_fragment_src.as_ptr(), std::ptr::null());
            gl::CompileShader(frag_id);

            // check for errors
            gl::GetShaderiv(frag_id, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                // get the error string back from the GPU to print it out
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

                // don't end just yet, in case anything else fails too
                failed = true;
            }

            // link the shaders together into a program
            prog_id = gl::CreateProgram();
            gl::AttachShader(prog_id, vert_id);
            gl::AttachShader(prog_id, frag_id);
            gl::LinkProgram(prog_id);

            // check for errors
            gl::GetProgramiv(prog_id, gl::LINK_STATUS, &mut success);
            if success == 0 {
                // get the error string back from the GPU to print it out
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

            // can delete the individual shader objects after the program is completed
            // whether compilation failed or not
            gl::DeleteShader(vert_id);
            gl::DeleteShader(frag_id);

            if failed {
                panic!("Shader program creation failed!");
            }
        }

        Self { id: prog_id }
    }

    /// activates this program
    pub fn r#use(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    /// deactivates this program
    pub fn unuse(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    /// gets the location of a uniform in the program given its name
    fn get_uniform_loc(&self, uniform_name: &str) -> i32 {
        // must be using the program currently or this fails
        self.r#use();

        let name = CString::new(uniform_name).unwrap();
        let mut uniform_location: i32 = 0;

        unsafe {
            uniform_location = gl::GetUniformLocation(self.id, name.as_ptr());
        }

        // unfortunate case: when we can't find the uniform
        // ! in the future, this may happen because the string being passed is not null-terminated
        if uniform_location < 0 {
            log::error!("uniform \"{}\" location couldn't be found", uniform_name)
        }

        uniform_location
    }

    /// updates a uniform in the program: float
    pub fn uniform1f(&self, uniform_name: &str, value: f32) {
        unsafe {
            self.r#use();
            let loc = self.get_uniform_loc(uniform_name);
            gl::Uniform1f(loc, value);
        }
    }

    /// updates a uniform in the program: vec2
    pub fn uniform2f(&self, uniform_name: &str, values: (f32, f32)) {
        unsafe {
            self.r#use();
            let loc = self.get_uniform_loc(uniform_name);
            gl::Uniform2f(loc, values.0, values.1);
        }
    }

    /// updates a uniform in the program: mat4
    ///
    /// must be given pointer to the first element of a contiguous matrix
    pub fn uniform_matrix4f(&self, uniform_name: &str, matrix_ptr: &f32) {
        unsafe {
            self.r#use();
            let loc = self.get_uniform_loc(uniform_name);
            gl::UniformMatrix4fv(loc, 1, gl::FALSE, matrix_ptr);
        }
    }
}
