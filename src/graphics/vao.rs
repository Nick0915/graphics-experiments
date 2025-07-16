use gl::{self, types::*};
use std::{mem, os::raw::c_void};

/// wrapper around a vertex array object for OpenGL
pub struct VAO {
    id: u32,
    attr_layout: Vec<u32>,
    r#type: GLenum,
}

impl VAO {
    /// create a new VAO given an attribute layout and the datatype of the vertex buffer.
    ///
    /// attr_layout will be a vec of u32s, each of which determine how many elements
    /// each attribute consists.
    ///
    /// for example, attr_layout=vec![2, 3] (assuming type is float) means the first attribute
    /// will be a vec2 (2 floats) and the second attribute will be a vec3 (3 floats)
    ///
    /// VertexDataType represents the type of each element (usually f32 for floats)
    pub fn new<VertexDataType>(attr_layout: Vec<u32>, r#type: GLenum) -> Self {
        let mut id = 0;

        unsafe {
            // generate and bind a vertex array
            gl::GenVertexArrays(1, &mut id);
            gl::BindVertexArray(id);

            let num_attrs = attr_layout.len();

            // the distance (in elements, not bytes) between each new vertex
            //
            // it is the distance between the first attr of the first vertex and
            // the first attr of the second vertex, which in turn is the sum of
            // the number of elements that occur in one vertex
            let stride = attr_layout.iter().sum::<u32>() * std::mem::size_of::<VertexDataType>() as u32;

            // will denote where each attribute starts
            let mut offset = 0;

            for index in 0..attr_layout.len() {
                // number of elements in this attribute
                let size = attr_layout[index];

                #[rustfmt::skip]
                gl::VertexAttribPointer(
                    index as u32,                                   // the N in layout (location = N)
                    size as i32,                                    // size (in quantity) of the data (vec3 -> 3 floats, so size = 3)
                    r#type,                                         // type of the data (vec3 uses floats, so gl::FLOAT)
                    gl::FALSE,                                      // normalized?
                    stride as i32,                                  // # of bytes between each new vertex (not to next attribute in the same vertex)
                    (offset * mem::size_of::<VertexDataType>()) as *const _      // offset (in bytes) to the first instance of this attr in the array (casted to a pointer)
                );
                // need to enable this attribute to make it work
                gl::EnableVertexArrayAttrib(id, index as u32);

                // update the next attribute's offset by moving forward by the
                // number of elements we just defined for the current attribute
                offset += attr_layout[index] as usize;
            }
        }

        Self {
            id,
            attr_layout,
            r#type,
        }
    }

    /// binds the VAO
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    /// unbinds all VAOs (including this one)
    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    /// frees up the VAO in gpu memory
    pub fn free(&mut self) {
        unsafe {
            self.unbind();
            gl::DeleteVertexArrays(1, &mut self.id);
        }
    }
}
