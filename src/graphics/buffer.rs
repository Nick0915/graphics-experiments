use gl::types::*;
use std::mem;

/// wrapper around (element-) array buffers for OpenGL
pub struct BufferObject {
    id: u32,
    target: GLenum,
    usage: GLenum,
}

impl BufferObject {
    /// creates a new (empty) buffer
    /// if type is ARRAY_BUFFER, this is a VBO
    /// if type is ELEMENT_ARRAY_BUFFER, this is an EBO/IBO
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

    /// binds the buffer type this object is associated to (EBO, IBO, etc)
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.target, self.id);
        }
    }

    /// unbinds the buffer type this object is associated to (EBO, IBO, etc)
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.target, 0);
        }
    }

    /// populates the buffer in the GPU with data
    pub fn buffer_data<T>(&mut self, data: &Vec<T>) {
        self.bind();
        unsafe {
            gl::BufferData(
                self.target,                // the buffer we're sending data to
                (data.len() * mem::size_of::<T>()) as GLsizeiptr,  // the number of bytes we're trying to allocate
                mem::transmute(&data[0]),   // pointer to the buffer (first element)
                self.usage,                 // how the buffer will be used (static, copy, etc.)
            );
        }
    }

    /// frees up the buffer in GPU memory
    pub fn free(&mut self) {
        unsafe {
            self.unbind();
            gl::DeleteBuffers(1, &mut self.id);
        }
    }
}
