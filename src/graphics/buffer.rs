use gl::types::*;
use std::mem;

pub struct BufferObject {
    id: u32,
    target: GLenum,
    usage: GLenum,
}

impl BufferObject {
    pub fn new(r#type: GLenum, usage: GLenum) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        };

        Self {
            id,
            target: r#type,
            usage,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.target, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.target, 0);
        }
    }

    pub fn buffer_data<T>(&mut self, data: &[T]) {
        self.bind();
        unsafe {
            gl::BufferData(
                self.target,
                (data.len() * mem::size_of::<T>()) as GLsizeiptr,
                mem::transmute(&data[0]),
                self.usage,
            );
        }
    }

    pub fn free(&mut self) {
        unsafe {
            self.unbind();
            gl::DeleteBuffers(1, &mut self.id);
        }
    }
}
