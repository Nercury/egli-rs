extern crate egli;
extern crate gl;
extern crate libc;
extern crate x11;

use egli::egl::{EGLint, EGL_HEIGHT, EGL_NONE, EGL_WIDTH};
use egli::error::Error;
use egli::{Context, Display, RenderableType, Surface, SurfaceType};
use gl::types::{GLchar, GLenum, GLint, GLuint};
use gl::{COLOR_ATTACHMENT0, FRAGMENT_INTERPOLATION_OFFSET_BITS, FRAGMENT_SHADER};
use std::ffi::CStr;
use std::{mem, ptr};

struct SafeProgram {
    program: GLuint,
}

impl SafeProgram {
    pub fn new() -> SafeProgram {
        let program = unsafe {
            let program = gl::CreateProgram();
            let e = gl::GetError();
            if e != gl::NO_ERROR {
                panic!("Failed to create GL program error: {}", e);
            }
            program
        };
        SafeProgram { program }
    }
}

impl Drop for SafeProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
            let e = gl::GetError();
            if e != gl::NO_ERROR {
                panic!("Failed to create GL program error: {}", e);
            }
        }
    }
}

const VTX_SHADER: &str = r#"#version 320 es
precision highp float;

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 texcoord;
out vec2 vTexcoord;

void main()
{
    gl_Position = vec4(position, 1.0);
    vTexcoord = texcoord;
}
"#;

const FRA_SHADER: &str = r#"#version 320 es
precision highp float;
layout(binding = 0) uniform highp sampler2D uTexture;

out vec4 color;
in vec2 vTexcoord;

void main() {
    color = texture(uTexture, vTexcoord);
    //color = vec4(1.0, 1.0, 0.0, 1.0);
}
"#;

struct Blitter {
    program: GLuint,
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
}

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        assert!(gl::GetError() == gl::NO_ERROR);

        shader = gl::CreateShader(ty);
        assert!(gl::GetError() == gl::NO_ERROR);

        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        assert!(gl::GetError() == gl::NO_ERROR);
        gl::CompileShader(shader);
        assert!(gl::GetError() == gl::NO_ERROR);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        assert!(gl::GetError() == gl::NO_ERROR);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            unsafe { gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len) };
            assert!(gl::GetError() == gl::NO_ERROR);
            assert!(len > 0);

            let mut buf = Vec::with_capacity(len as usize);
            let buf_ptr = buf.as_mut_ptr() as *mut gl::types::GLchar;
            unsafe {
                gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), buf_ptr);
                assert!(gl::GetError() == gl::NO_ERROR);
                buf.set_len(len as usize);
            };

            match String::from_utf8(buf) {
                Ok(log) => {
                    panic!("Error log: {}", log);
                }
                Err(vec) => {
                    panic!("Could not convert compilation log from buffer: {}", vec);
                }
            }
        }
        shader
    }
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetProgramInfoLog(program, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len as usize);
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }
        program
    }
}

impl Blitter {
    fn new(display: &Display, surface: &Surface, context: &Context) -> Blitter {
        display
            .make_current(&surface, &surface, &context)
            .expect("make current failed");

        let vertex_shader = compile_shader(VTX_SHADER, gl::VERTEX_SHADER);
        let fragment_shader = compile_shader(FRA_SHADER, gl::FRAGMENT_SHADER);
        let program = link_program(vertex_shader, fragment_shader);

        let mut vao = 0; // Vertex Attribute Object
        let mut vbo = 0;
        let mut ebo = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            assert!(gl::GetError() == gl::NO_ERROR);
            gl::GenBuffers(1, &mut vbo);
            assert!(gl::GetError() == gl::NO_ERROR);
            gl::GenBuffers(1, &mut ebo);
            assert!(gl::GetError() == gl::NO_ERROR);

            gl::BindVertexArray(vao);
            assert!(gl::GetError() == gl::NO_ERROR);

            #[rustfmt::skip]
            let vertices = vec![
                -1.0f32, -1.0, 0f32, 0.0, 1.0,
                -1.0, 1.0, 0f32, 0.0, 0.0,
                1.0, 1.0, 0f32, 1.0, 0.0,
                1.0, -1.0, 0f32, 1.0, 1.0,
            ];
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            assert!(gl::GetError() == gl::NO_ERROR);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );
            assert!(gl::GetError() == gl::NO_ERROR);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            assert!(gl::GetError() == gl::NO_ERROR);
            let indices: Vec<u8> = vec![0, 1, 2, 0, 2, 3];
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<u8>()) as isize,
                indices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );
            assert!(gl::GetError() == gl::NO_ERROR);

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                5 * mem::size_of::<f32>() as i32,
                ptr::null(),
            );
            assert!(gl::GetError() == gl::NO_ERROR);

            gl::EnableVertexAttribArray(0);
            assert!(gl::GetError() == gl::NO_ERROR);

            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                5 * mem::size_of::<f32>() as i32,
                (3 * mem::size_of::<f32>()) as *const c_void,
            );
            assert!(gl::GetError() == gl::NO_ERROR);

            gl::EnableVertexAttribArray(1);
            assert!(gl::GetError() == gl::NO_ERROR);

            gl::BindVertexArray(0);
            assert!(gl::GetError() == gl::NO_ERROR);
        }
        Blitter {
            program,
            vao,
            vbo,
            ebo,
        }
    }

    fn blit_texture(
        &self,
        texture_handle: GLuint,
        display: &Display,
        surface: &Surface,
        context: &Context,
    ) {
        display
            .make_current(&surface, &surface, &context)
            .expect("make current failed");
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture_handle);
            assert!(gl::GetError() == gl::NO_ERROR);

            gl::UseProgram(self.program);
            assert!(gl::GetError() == gl::NO_ERROR);

            gl::BindVertexArray(self.vao);
            assert!(gl::GetError() == gl::NO_ERROR);

            gl::Enable(gl::BLEND);
            assert!(gl::GetError() == gl::NO_ERROR);

            // typical blend
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            assert!(gl::GetError() == gl::NO_ERROR);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_BYTE, ptr::null());
            assert!(gl::GetError() == gl::NO_ERROR);

            gl::BindVertexArray(0);
            assert!(gl::GetError() == gl::NO_ERROR);
        }
    }
}

// Render to texture in a specific GL context
fn render_to_texture(
    display: &Display,
    surface: &Surface,
    context: &Context,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
) -> GLuint {
    display
        .make_current(&surface, &surface, &context)
        .expect("make current failed");
    let texture_handle = unsafe {
        let mut frame_buffer: GLuint = 0;
        gl::GenFramebuffers(1, &mut frame_buffer);
        assert!(gl::GetError() == gl::NO_ERROR);

        gl::BindFramebuffer(gl::FRAMEBUFFER, frame_buffer);
        assert!(gl::GetError() == gl::NO_ERROR);

        let mut texture_handle: GLuint = 0;
        gl::GenTextures(1, &mut texture_handle);
        assert!(gl::GetError() == gl::NO_ERROR);
        gl::BindTexture(gl::TEXTURE_2D, texture_handle);
        assert!(gl::GetError() == gl::NO_ERROR);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);

        #[rustfmt::skip]
        let debug_texture = vec!(
            255u8, 0, 0, 255,
            0, 255, 0, 255,
            255, 255, 0, 255,
            255, 255, 255, 255,
        );

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            640,
            480,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            ptr::null(),
            //debug_texture.as_ptr() as *const c_void,
        );
        assert!(gl::GetError() == gl::NO_ERROR);

        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            texture_handle,
            0,
        );
        assert!(gl::GetError() == gl::NO_ERROR);

        // Always check framebuffer status
        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            //return Err(Error))
            println!("framebuffer not complete...");
            // have to clean up allocated resources as well.
            // this really is highly unsafe.
            return 0;
        }

        gl::ClearColor(1.0, 1.0, 1.0, 0.0);
        assert!(gl::GetError() == gl::NO_ERROR);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        assert!(gl::GetError() == gl::NO_ERROR);

        // // we can now render a simple rect into the texture to be composed into the primary context
        gl::Enable(gl::SCISSOR_TEST);
        assert!(gl::GetError() == gl::NO_ERROR);

        gl::Scissor(x, y, w, h);
        assert!(gl::GetError() == gl::NO_ERROR);

        gl::ClearColor(r, g, b, a);
        assert!(gl::GetError() == gl::NO_ERROR);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        assert!(gl::GetError() == gl::NO_ERROR);

        gl::Disable(gl::SCISSOR_TEST);
        assert!(gl::GetError() == gl::NO_ERROR);

        // release the framebuffer handle.
        // It's up to client code to release the allocated texture
        gl::DeleteFramebuffers(1, &frame_buffer);
        assert!(gl::GetError() == gl::NO_ERROR);

        gl::BindTexture(gl::TEXTURE_2D, 0);
        assert!(gl::GetError() == gl::NO_ERROR);

        texture_handle
    };
    texture_handle
}

fn clear_surface(display: &Display, surface: &Surface, context: &Context) {
    display
        .make_current(&surface, &surface, &context)
        .expect("make current failed");
    unsafe {
        gl::ClearColor(0.0, 0.0, 1.0, 1.0);
        assert!(gl::GetError() == gl::NO_ERROR);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        assert!(gl::GetError() == gl::NO_ERROR);
    }
}

fn main() {
    println!("This example demonstrates using multiple GL contexts with shared data objecs created via EGL.");
    println!("This example requires GL, EGL and xlib installed.");
    println!("On Ubuntu they are named `libegl1-mesa-dev`, `libx11-dev`.");

    let display_and_window = X11DisplayAndWindow::new("Hello EGL", 640, 480);

    let egl_display = Display::from_display_id(display_and_window.display as *mut _)
        .expect("failed to get EGL display");

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
        .with_depth_size(24)
        .with_surface_type(SurfaceType::WINDOW)
        .with_renderable_type(RenderableType::OPENGL_ES2)
        //.with_conformant(RenderableType::e)
        .choose_configs()
        .expect("failed to get configurations");

    let first_config = *configs
        .first()
        .expect("no compatible EGL configuration was found");

    let surface = egl_display
        .create_window_surface(first_config, display_and_window.window as *mut _)
        .expect("failed to create window surface");

    // Create a primary GL context that will be used to compose textures
    // rendered by other, independent GL contexts.
    let primary_context = egl_display
        .create_context_with_client_version(first_config, egli::ContextClientVersion::OpenGlEs2)
        .expect("failed to create OpenGL context");

    // Create a secondary context, Context A that will do a separate render pass.
    // The advantage of using a separate context is that the complex GL state machine
    // can't be polluted by rendering in other GL contexts. This pollution can be difficult to debug.
    //let secondary_context_attrs: [EGLint; 5] = [EGL_WIDTH, 640, EGL_HEIGHT, 480, EGL_NONE];
    let secondary_context_attrs: [EGLint; 1] = [EGL_NONE];
    let secondary_context_a = egl_display
        .create_shared_context(
            &primary_context,
            first_config,
            Some(&secondary_context_attrs),
        )
        .expect("Failed to create shared GL context.");

    let secondary_context_b = egl_display
        .create_shared_context(
            &primary_context,
            first_config,
            Some(&secondary_context_attrs),
        )
        .expect("Failed to create shared GL context.");

    // Create a surface the secondary context can use
    let pbuffer_attrs: [EGLint; 5] = [EGL_WIDTH, 640, EGL_HEIGHT, 480, EGL_NONE];
    let secondary_surface = egl_display
        .create_pbuffer_surface(first_config, &pbuffer_attrs)
        .expect("Failed to create pbuffer based surface.");

    let pbuffer_attrs: [EGLint; 5] = [EGL_WIDTH, 640, EGL_HEIGHT, 480, EGL_NONE];
    let surface_b = egl_display
        .create_pbuffer_surface(first_config, &pbuffer_attrs)
        .expect("Failed to create pbuffer based surface.");

    gl::load_with(|s| unsafe { mem::transmute(egli::egl::get_proc_address(s)) });

    // Create a blitter on the primary GL context. This allows us to blit
    // textures created by shared (secondary) GL contexts into the prmary GL context.
    let blitter = Blitter::new(&egl_display, &surface, &primary_context);

    display_and_window.wait_for_close(move || {
        // Use the secondary context to render to a texture. This texture
        // should be sharable between the secondary and primary context.
        let render_a_texture = render_to_texture(
            &egl_display,
            &secondary_surface,
            &secondary_context_a,
            0,
            0,
            320,
            240,
            0.0,
            1.0,
            0.0,
            1.0,
        );

        let render_b_texture = render_to_texture(
            &egl_display,
            &secondary_surface,
            &secondary_context_b,
            320,
            240,
            100,
            100,
            0.0,
            1.0,
            1.0,
            1.0,
        );

        // clear the primary GL context before we blit anything into it.
        clear_surface(&egl_display, &surface, &primary_context);

        blitter.blit_texture(render_a_texture, &egl_display, &surface, &primary_context);
        blitter.blit_texture(render_b_texture, &egl_display, &surface, &primary_context);

        egl_display
            .swap_buffers(&surface)
            .expect("failed to swap buffers");
    });
}

use std::ffi::CString;
use std::mem::zeroed;
use std::os::raw::{c_uint, c_void};
use std::ptr::{null, null_mut};
use x11::xlib;

/// Minimal helper to initialize X11 display and window.
struct X11DisplayAndWindow {
    pub window: std::os::raw::c_ulong,
    pub display: *mut x11::xlib::Display,
    wm_delete_window: std::os::raw::c_ulong,
    wm_protocols: std::os::raw::c_ulong,
}

impl X11DisplayAndWindow {
    pub fn new(
        title: &'static str,
        default_width: c_uint,
        default_height: c_uint,
    ) -> X11DisplayAndWindow {
        let window;
        let display;
        let wm_delete_window;
        let wm_protocols;

        unsafe {
            // Open display
            display = xlib::XOpenDisplay(null());
            if display == null_mut() {
                panic!("can't open display");
            }

            // Load atoms
            let wm_delete_window_str = CString::new("WM_DELETE_WINDOW").unwrap();
            let wm_protocols_str = CString::new("WM_PROTOCOLS").unwrap();

            wm_delete_window =
                xlib::XInternAtom(display, wm_delete_window_str.as_ptr(), xlib::False);
            wm_protocols = xlib::XInternAtom(display, wm_protocols_str.as_ptr(), xlib::False);

            if wm_delete_window == 0 || wm_protocols == 0 {
                panic!("can't load atoms");
            }

            // Create window
            let screen_num = xlib::XDefaultScreen(display);
            let root = xlib::XRootWindow(display, screen_num);
            let white_pixel = xlib::XWhitePixel(display, screen_num);

            let mut attributes: xlib::XSetWindowAttributes = zeroed();
            attributes.background_pixel = white_pixel;

            window = xlib::XCreateWindow(
                display,
                root,
                0,
                0,
                default_width,
                default_height,
                0,
                0,
                xlib::InputOutput as c_uint,
                null_mut(),
                xlib::CWBackPixel,
                &mut attributes,
            );

            // Set window title
            let title_str = CString::new(title).unwrap();
            xlib::XStoreName(display, window, title_str.as_ptr() as *mut _);
        }

        X11DisplayAndWindow {
            window: window,
            display: display,
            wm_delete_window: wm_delete_window,
            wm_protocols: wm_protocols,
        }
    }

    pub fn wait_for_close<F>(&self, render: F)
    where
        F: Fn(),
    {
        unsafe {
            // Subscribe to delete (close) events
            let mut protocols = [self.wm_delete_window];

            if xlib::XSetWMProtocols(
                self.display,
                self.window,
                &mut protocols[0] as *mut xlib::Atom,
                1,
            ) == xlib::False
            {
                panic!("can't set WM protocols");
            }

            // Show window
            xlib::XMapWindow(self.display, self.window);

            // Main loop
            let mut event: xlib::XEvent = zeroed();

            'main: loop {
                while xlib::XPending(self.display) > 0 {
                    xlib::XNextEvent(self.display, &mut event);
                    match event.get_type() {
                        xlib::ClientMessage => {
                            let xclient: xlib::XClientMessageEvent = From::from(event);

                            // WM_PROTOCOLS client message
                            if xclient.message_type == self.wm_protocols && xclient.format == 32 {
                                let protocol = xclient.data.get_long(0) as xlib::Atom;

                                // WM_DELETE_WINDOW (close event)
                                if protocol == self.wm_delete_window {
                                    break 'main;
                                }
                            }
                        }

                        _ => {}
                    }
                }

                render();
            }
        }
    }
}

impl Drop for X11DisplayAndWindow {
    fn drop(&mut self) {
        unsafe {
            xlib::XDestroyWindow(self.display, self.window);
            xlib::XCloseDisplay(self.display);
        }
    }
}
