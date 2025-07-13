use gl;
use log;
use std;

mod util;
mod window;

fn main() {
    let mut window = window::Window::init();

    while !window.should_close() {
        window.update();
    }
}


