use gl::{self, types::*};
use std::{mem, os::raw::c_void};

pub struct VAO {
    id: u32,
    attr_layout: Vec<u32>,
    r#type: GLenum,
}

impl VAO {
    pub fn new<T>(attr_layout: Vec<u32>, r#type: GLenum, data: &Vec<T>) -> Self {
        let mut id = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut id);
            gl::BindVertexArray(id);

            let num_attrs = attr_layout.len();

            let stride = attr_layout.iter().sum::<u32>() * std::mem::size_of::<T>() as u32;

            let mut offset = 0;
            for index in 0..attr_layout.len() {
                let size = attr_layout[index];

                #[rustfmt::skip]
                gl::VertexAttribPointer(
                    index as u32,                                   // the N in layout (location = N)
                    size as i32,                                    // size (in quantity) of the data (vec3 -> 3 floats, so size = 3)
                    r#type,                                         // type of the data (vec3 uses floats, so gl::FLOAT)
                    gl::FALSE,                                      // normalized?
                    stride as i32,                                  // # of bytes between each new vertex (not to next attribute in the same vertex)
                    (offset * mem::size_of::<T>()) as *const _      // offset (in bytes) to the first instance of this attr in the array (casted to a pointer)
                );
                gl::EnableVertexArrayAttrib(id, index as u32);

                offset += attr_layout[index] as usize;
            }
        }

        Self {
            id,
            attr_layout,
            r#type,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn free(&mut self) {
        unsafe {
            self.unbind();
            gl::DeleteVertexArrays(1, &mut self.id);
        }
    }
}
