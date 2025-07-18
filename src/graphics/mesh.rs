use std::collections::{HashMap, HashSet};

use crate::{graphics::Drawable, *};
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
        // 1. create VAO, VBO, IBO
        let mut vertex_array = graphics::VAO::new(gl::FLOAT);
        let mut vertex_buffer = graphics::BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
        let mut index_buffer = graphics::BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);

        // 2. bind VAO
        vertex_array.bind();

        // 3. bind IBO and populate
        index_buffer.bind();
        index_buffer.buffer_data(&indices);

        // 4. bind VBO and populate
        vertex_buffer.bind();
        vertex_buffer.buffer_data(&vertices);

        // 5. set VAO layout
        vertex_array.set_attr_layout::<f32>(attr_layout);

        Self {
            vertex_array,
            vertex_buffer,
            index_buffer,
            num_indices: indices.len(),
            shader_program,
        }
    }

    /// parses an OBJ file and returns a vertex buffer object and index buffer object
    pub fn from_obj(obj: &str, shader_program: ShaderProgram) -> Self {
        let mut positions = Vec::new();
        let mut texcoords = Vec::new();
        let mut normals = Vec::new();

        let mut vertex_data: Vec<f32> = Vec::new();
        let mut vertex_map: HashMap<(u32, u32, u32), u32> = HashMap::new();
        let mut indices: Vec<u32> = Vec::new();

        let mut num_vertices = 0;

        for (line_no, line) in obj.lines().enumerate() {
            let mut tokens = line.split_whitespace();
            match tokens.next().unwrap() {
                "v" => {
                    let position = (
                        tokens.next().unwrap().parse::<f32>().unwrap(),
                        tokens.next().unwrap().parse::<f32>().unwrap(),
                        tokens.next().unwrap().parse::<f32>().unwrap(),
                    );

                    positions.push(position);
                },
                "vn" => {
                    let normal = (
                        tokens.next().unwrap().parse::<f32>().unwrap(),
                        tokens.next().unwrap().parse::<f32>().unwrap(),
                        tokens.next().unwrap().parse::<f32>().unwrap(),
                    );

                    normals.push(normal);
                },
                "vt" => {
                    let texcoord = (
                        tokens.next().unwrap().parse::<f32>().unwrap(),
                        tokens.next().unwrap().parse::<f32>().unwrap(),
                    );

                    texcoords.push(texcoord);
                },
                "f" => {
                    // cannot let the vector consume the iterator here because we
                    // need to borrow the iterator later to check if it's empty or not
                    let tokens = tokens.by_ref().collect::<Vec<&str>>();
                    for i in 1..(tokens.len() - 1) {
                        let token_set = vec![tokens[0], tokens[i], tokens[i + 1]];
                        for token in token_set {
                            if token.contains('/') {
                                let idxs = token.split('/').collect::<Vec<&str>>();
                                let (pos_i, tex_i, norm_i) = (
                                    idxs[0].parse::<u32>().unwrap() - 1,
                                    idxs[1].parse::<u32>().unwrap() - 1,
                                    idxs[2].parse::<u32>().unwrap() - 1,
                                );

                                if !vertex_map.contains_key(&(pos_i, tex_i, norm_i)) {
                                    vertex_map.insert((pos_i, tex_i, norm_i), num_vertices);
                                    vertex_data.append(&mut vec![
                                        positions[pos_i as usize].0,
                                        positions[pos_i as usize].1,
                                        positions[pos_i as usize].2,
                                        texcoords[tex_i as usize].0,
                                        texcoords[tex_i as usize].1,
                                        normals[norm_i as usize].0,
                                        normals[norm_i as usize].1,
                                        normals[norm_i as usize].2,
                                    ]);
                                    num_vertices += 1;
                                }

                                indices.push(vertex_map[&(pos_i, tex_i, norm_i)]);
                            }
                        }
                    }
                },
                _ => {
                    // consume all the tokens in any ignored line so the later
                    // when we check for left-over tokens, we don't false positive
                    // on these ignored lines (e.g. comments, "o <object name>", etc
                    while { match tokens.next() {
                        Some(_) => true,
                        None => false,
                    } } {}
                },
            }

            match tokens.next() {
                Some(tok) => {
                    let mut leftover = tokens.collect::<Vec<&str>>();
                    leftover.insert(0, tok);
                    log::error!("Left over token(s) when parsing line {} (\"{}\"): {:?}", line_no + 1, line, leftover);
                    panic!();
                },
                None => {},
            }
        }

        // 1. create VAO, VBO, IBO
        let mut vertex_array = VAO::new(gl::FLOAT);
        let mut vertex_buffer = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
        let mut index_buffer = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);

        // 2. bind VAO
        vertex_array.bind();

        // 3. bind IBO and populate
        index_buffer.bind();
        index_buffer.buffer_data(&indices);

        // 4. bind VBO and populate
        vertex_buffer.bind();
        vertex_buffer.buffer_data(&vertex_data);

        // 5. set VAO layout
        vertex_array.set_attr_layout::<f32>(vec![3, 2, 3]); // vec3 pos, vec2 texcoord, vec3 normal

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
    /// currently only works with wavefront schemes that only contains vertices and
    /// triangle faces (no UVs, normals, etc., quads)
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

    /// updates a uniform in this mesh's shader program: mat4
    pub fn uniform_matrix4f(&self, uniform_name: &str, matrix_ptr: &f32) {
        self.shader_program.uniform_matrix4f(uniform_name, matrix_ptr);
    }

    /// updates a uniform in this mesh's shader program: float
    pub fn uniform1f(&self, uniform_name: &str, value: f32) {
        self.shader_program.uniform1f(uniform_name, value);
    }

    /// updates a uniform in the this mesh's shader program: vec2
    pub fn uniform2f(&self, uniform_name: &str, values: (f32, f32)) {
        self.shader_program.uniform2f(uniform_name, values);
    }

    /// draws a mesh onto the framebuffer
    pub fn draw(&self) {
        // bind all appropriate buffers/arrays/programs
        self.vertex_array.bind();
        self.index_buffer.bind();
        self.vertex_buffer.bind();
        self.shader_program.r#use();

        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.num_indices as i32, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}
