use crate::*;

pub struct Mesh {
    vertex_array: graphics::VAO,
    vertex_buffer: graphics::BufferObject,
    index_buffer: graphics::BufferObject,
    num_indices: usize,
    shader_program: graphics::ShaderProgram,
}

impl Mesh {
    pub fn from_list(vertices: Vec<f32>, indices: Vec<u32>, shader_program: graphics::ShaderProgram) -> Self {
        let mut vertex_buffer = graphics::BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
        vertex_buffer.buffer_data(&vertices);

        let mut index_buffer = graphics::BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
        index_buffer.buffer_data(&indices);

        let attr_layout = vec![3];
        let mut vertex_array = graphics::VAO::new(attr_layout, gl::FLOAT, &vertices);

        log::info!("Mesh has {} vertices and {} faces", (vertices.len() / 3) as i32, (indices.len() / 3) as i32);
        log::info!("indices: {:?}", indices);

        Self {
            vertex_array,
            vertex_buffer,
            index_buffer,
            num_indices: (indices.len() / 3) as usize,
            shader_program,
        }
    }

    pub fn from_obj(obj: &str, shader_program: graphics::ShaderProgram) -> Self {
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

        Self::from_list(vertices, indices, shader_program)
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
