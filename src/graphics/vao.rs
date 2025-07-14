use gl::{self, types::*};
use std::mem;

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
                let pointer = mem::transmute(&data[offset]);
                offset += attr_layout[index] as usize;

                gl::VertexAttribPointer(
                    index as u32,
                    size as i32,
                    r#type,
                    gl::FALSE,
                    stride as i32,
                    pointer
                );

                gl::EnableVertexArrayAttrib(id, index as u32);
            }
        }

        Self { id, attr_layout, r#type }
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
