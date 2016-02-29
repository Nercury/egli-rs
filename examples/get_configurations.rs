extern crate egli;

use egli::Display;
use egli::renderable;

fn main() {
    let display = Display::from_default_display()
        .expect("failed to get EGL display");

    let egl_version = display.initialize_and_get_version()
        .expect("failed to initialize EGL");
    println!("Using EGL {}", egl_version);

    let configs = display.config_filter()
        .with_blue_size(8)
        .with_alpha_size(8)
        .with_buffer_size(32)
        .with_depth_size(32)
        .with_conformant(renderable::OPENGL_ES2)
        .choose_configs()
        .expect("failed to get configurations");

    println!("There are {:#?} display configurations", configs);
}
