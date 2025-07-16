use crate::*;

pub struct Mesh {
    vertex_array: graphics::VAO,
    vertex_buffer: graphics::BufferObject,
    index_buffer: graphics::BufferObject,
    num_indices: usize,
    shader_program: graphics::ShaderProgram,
}

impl Mesh {
    pub fn from_list(vertices: Vec<f32>, attr_layout: Vec<u32>, indices: Vec<u32>, shader_program: graphics::ShaderProgram) -> Self {
        let mut index_buffer = graphics::BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
        index_buffer.bind();
        index_buffer.buffer_data(&indices);

        let mut vertex_buffer = graphics::BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
        vertex_buffer.bind();
        vertex_buffer.buffer_data(&vertices);

        // let attr_layout = vec![3];
        let mut vertex_array = graphics::VAO::new::<f32>(attr_layout, gl::FLOAT);
        vertex_array.bind();

        log::info!("Mesh has {} vertices and {} faces", (vertices.len() / 3) as i32, (indices.len() / 3) as i32);

        Self {
            vertex_array,
            vertex_buffer,
            index_buffer,
            num_indices: indices.len(),
            shader_program,
        }
    }

    pub fn from_basic_obj(obj: &str, shader_program: graphics::ShaderProgram) -> Self {
        let mut vertices: Vec<f32> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let mut obj_name: String = String::new();

        for line in obj.lines() {
            match line.chars().nth(0).unwrap() {
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
                'o' => {
                    obj_name = line.split_whitespace().nth(1).unwrap().to_string()
                }
                '#' | _ => {},
            }
        }

        Self::from_list(vertices, vec![3], indices, shader_program)
    }

    pub fn draw(&self) {
        self.vertex_buffer.bind();
        self.index_buffer.bind();
        self.vertex_array.bind();
        self.shader_program.r#use();

        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.num_indices as i32, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}
