extern crate egli;
extern crate gl;
extern crate libc;
extern crate x11;

use egli::egl::EGLint;
use egli::egl::{EGL_HEIGHT, EGL_NONE, EGL_WIDTH};
use egli::{Display, RenderableType, SurfaceType};
use std::mem;
use std::os::raw::c_void;
use std::{thread, time};

fn main() {
    println!("This example requires GL, EGL installed.");
    println!("On Ubuntu it's named `libegl1-mesa-dev`.");

    let egl_display = Display::from_default_display().expect("Failed to get default EGL display.");

    println!(
        "Using EGL {}",
        egl_display
            .initialize_and_get_version()
            .expect("failed to initialize")
    );

    let configs = egl_display
        .config_filter()
        .with_red_size(8)
        .with_green_size(8)
        .with_blue_size(8)
        .with_alpha_size(8)
        .with_depth_size(24)
        .with_surface_type(SurfaceType::PBUFFER)
        .with_renderable_type(RenderableType::OPENGL_ES2)
        .choose_configs()
        .expect("failed to get configurations");

    println!("{} Configs available. Using the first.", configs.len());

    let first_config = *configs
        .first()
        .expect("no compatible EGL configuration was found");

    let pbuffer_attrs: [EGLint; 5] = [EGL_WIDTH, 640, EGL_HEIGHT, 480, EGL_NONE];

    let surface = egl_display
        .create_pbuffer_surface(first_config, &pbuffer_attrs)
        .expect("Failed to create pbuffer based surface.");

    let context = egl_display
        .create_context(first_config)
        .expect("failed to create OpenGL context");

    egl_display
        .make_current(&surface, &surface, &context)
        .expect("make current failed");

    gl::load_with(|s| unsafe { mem::transmute(egli::egl::get_proc_address(s)) });

    for i in 1..5 {
        println!("Frame {}", i);
        unsafe {
            gl::Viewport(0, 0, 640, 480);
            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        egl_display
            .swap_buffers(&surface)
            .expect("failed to swap buffers");

        // get some pixels
        let mut pixels = vec![0u8; 640*480*4];
        unsafe {
            gl::ReadPixels(0, 0, 640, 480, gl::RGBA, gl::UNSIGNED_BYTE, pixels.as_mut_ptr() as *mut c_void);
        }

        println!("{}:{}:{}:{}", pixels[0], pixels[1], pixels[2], pixels[3]);

        thread::sleep(time::Duration::from_secs(1));
    }
}
