/// get the vertices (Vec<f32>), attribute layout (Vec<u32>), and indices (Vec<u32>)
/// of a hard-coded default plane
pub fn default_plane() -> (Vec<f32>, Vec<u32>, Vec<u32>) {
    #[rustfmt::skip]
    let vertices: Vec<f32> = vec![
        //  x,    y,    z
         -1.0, -1.0,  0.0, // 0: bottom-left
          1.0, -1.0,  0.0, // 1: bottom-right
         -1.0,  1.0,  0.0, // 2: top-left
          1.0,  1.0,  0.0, // 3: top-right
    ];

    // pos, color
    let attr_layout = vec![3];

    #[rustfmt::skip]
    let indices: Vec<u32> = vec![
        1, 2, 0,    // top-left tri
        1, 3, 2     // bottom-right tri
    ];

    (vertices, attr_layout, indices)
}
