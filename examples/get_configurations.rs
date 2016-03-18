extern crate egli;

use egli::Display;
use egli::renderable;

fn main() {
    println!("This example requires EGL library installed.");
    println!("On Ubuntu it is named `libegl1-mesa-dev`.");

    println!("Supported EGL extensions: {}", egli::query_extensions()
        .expect("failed to query EGL extensions"));

    let display = Display::from_default_display()
        .expect("failed to get EGL display");

    println!("Using EGL {}", display.initialize_and_get_version().expect("failed to initialize"));

    println!(
        "\
        Supported APIs: {apis}\n\
        Display extensions: {extensions}\n\
        EGL version: {version}\n\
        Vendor: {vendor}\
        ",
        apis = display.query_client_apis().expect("failed to query display"),
        extensions = display.query_extensions().expect("failed to query display"),
        version = display.query_version().expect("failed to query display"),
        vendor = display.query_vendor().expect("failed to query display"),
    );

    let configs = display.config_filter()
        .with_blue_size(8)
        .with_alpha_size(8)
        .with_buffer_size(32)
        .with_depth_size(32)
        .with_conformant(renderable::OPENGL_ES2)
        .choose_configs()
        .expect("failed to get configurations");

    println!("There are {} display configurations", configs.len());
    println!("First found configuration mathing parameters is: {:#?}", configs.first());
}
