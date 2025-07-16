use crate::*;

/// wrapper around the objects needed to draw a mesh for OpenGL
/// specifically, uses the glDrawElements with GL_TRIANGLES
pub struct Mesh {
    vertex_array: graphics::VAO,
    vertex_buffer: graphics::BufferObject,
    index_buffer: graphics::BufferObject,
    num_indices: usize,
    shader_program: graphics::ShaderProgram,
}

impl Mesh {
    /// builds a mesh from a list of vertices, indices, an attribute layout scheme, and a shader program
    pub fn from_list(vertices: Vec<f32>, attr_layout: Vec<u32>, indices: Vec<u32>, shader_program: graphics::ShaderProgram) -> Self {
        // create IBO
        let mut index_buffer = graphics::BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
        index_buffer.bind();
        index_buffer.buffer_data(&indices);

        // create VBO
        let mut vertex_buffer = graphics::BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
        vertex_buffer.bind();
        vertex_buffer.buffer_data(&vertices);

        // create VAO
        let mut vertex_array = graphics::VAO::new::<f32>(attr_layout, gl::FLOAT);
        vertex_array.bind();

        Self {
            vertex_array,
            vertex_buffer,
            index_buffer,
            num_indices: indices.len(),
            shader_program,
        }
    }

    /// creates a mesh from a waveform .obj style string (NOT A FILE)
    ///
    /// currently only works with waveform schemes that only contains vertices and
    /// triangle faces (no UVs, colors, normals, etc., quads)
    pub fn from_basic_obj(obj: &str, shader_program: graphics::ShaderProgram) -> Self {
        // will buffer in the values to these vectors
        let mut vertices: Vec<f32> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let mut obj_name: String = String::new();

        // go line by line
        for line in obj.lines() {
            // first letter of the line tells you what it mans
            match line.chars().nth(0).unwrap() {
                // vertex (3 floats: x, y, z)
                'v' => {
                    for token in line.split_whitespace() {
                        match token.parse::<f32>() {
                            Ok(coord) => {
                                vertices.push(coord)
                            },
                            _ => {},
                        }
                    }
                },
                // triangle/face (3 indices: v1, v2, v3)
                'f' => {
                    for token in line.split_whitespace() {
                        match token.parse::<u32>() {
                            Ok(index) => {
                                indices.push(index - 1)
                            },
                            _ => {},
                        }
                    }
                },
                // object name
                'o' => {
                    obj_name = line.split_whitespace().nth(1).unwrap().to_string()
                }
                // comment (ignore this line)
                '#' | _ => {},
            }
        }

        // we have created a list of vertices and indices, so pass it to the other function
        Self::from_list(vertices, vec![3], indices, shader_program)
    }

    /// draws a mesh onto the framebuffer
    pub fn draw(&self) {
        // bind all appropriate buffers/arrays/programs
        self.vertex_buffer.bind();
        self.index_buffer.bind();
        self.vertex_array.bind();
        self.shader_program.r#use();

        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.num_indices as i32, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}
