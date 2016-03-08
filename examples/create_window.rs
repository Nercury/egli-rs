extern crate egli;

use std::ptr;
use std::thread;
use egli::{ Display, renderable };

fn main() {
    let display = Display::from_default_display()
        .expect("failed to get EGL display");

    println!("Using EGL {}", display.initialize_and_get_version().expect("failed to initialize"));

    let config = display.config_filter()
        .with_red_size(4)
        .with_green_size(4)
        .with_blue_size(4)
        .choose_configs()
        .expect("failed to get configurations")
        .first()
        .expect("no compatible EGL configuration was found")
        .clone();

    let surface = display.create_window_surface(config, ptr::null_mut())
        .expect("failed to create window surface");

    thread::sleep_ms(1000);
}
