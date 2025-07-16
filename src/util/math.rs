
#[inline]
pub fn deg2rad(deg: f32) -> f32 {
    (deg * std::f32::consts::PI) / 180.
}

#[inline]
pub fn rad2deg(rad: f32) -> f32 {
    (rad * 180.) / std::f32::consts::PI
}

pub fn pretty_print_mat4(message: &str, mat: &glam::Mat4) {
    println!("{}: ", message);
    for i in 0..4 {
        if i != 0 {
            println!()
        }
        print!("[");
        for j in 0..4 {
            if j != 0 {
                print!(", ");
            }
            print!("{:^6.2}", mat.col(j)[i]);
        }
        print!("]");
    }
    println!();
}
