## EGLI - EGL Interface for Rust

[![Build Status](https://travis-ci.org/Nercury/egli-rs.svg?branch=master)](https://travis-ci.org/Nercury/egli-rs)

## At a glance

```rust
extern crate egli;

use egli::{ Display, RenderableType };

fn main() {
    let display = Display::from_default_display()
        .expect("failed to get EGL display");

    display.initialize()
        .expect("failed to initialize");

    let configs = display.config_filter()
                         .with_blue_size(8)
                         .with_red_size(8)
                         .with_green_size(8)
                         .with_depth_size(24)
                         .with_conformant(RenderableType::OPENGL_ES2)
                         .choose_configs()
                         .expect("failed to get configurations");

    println!("There are {} display configurations", configs.len());
    println!("First found configuration matching parameters is: {:#?}",
             configs.first());

    // proceed to create surface & context ...
}
```

- Browse [api reference here](http://nercury.github.io/egli-rs/egli/index.html).
- See OpenGL [example with X11 window here](https://github.com/Nercury/egli-rs/blob/master/examples/x11_gl_window.rs).

### Is it finished?

No, and it may take considerable time to get it into the state where
we could name it "finished". However, the subset that is implemented
should be ok to use and should not change drastically, except the way
`egli::ffi` is bound.

Some things that would be nice to have:

- More complete API coverage in higher-level `egli`.
- An easy way to decorate error results with additional info from `eglGetError`.

### What is EGL

EGL is a window system-independent equivalent to the GLX and WGL APIs, which respectively enable OpenGL support in X and Microsoft Windows. It is an interface between Khronos rendering APIs such as OpenGL ES or OpenVG and the underlying native platform window system. It handles graphics context management, surface/buffer binding, and rendering synchronization and enables high-performance, accelerated, mixed-mode 2D and 3D rendering using other Khronos APIs.

### Why use EGLI

Many libraries such as SDL or glutin already do what EGL does (and more). Usually they are using EGL behind the scenes. So using EGL directly only makes sense if:

- You obtained window/display handle by other means.
- You need complete control of surface configuration.
- You need to initialize OpenGL context.
- You already have another OpenGL library that needs to call `get_proc_address` which EGL provides.
- You need a way to swap buffers at the end of the scene.

## EGLI Details

Browse [api reference here](http://nercury.github.io/egli-rs/egli/index.html).

EGLI has two abstraction levels.

Lower level EGL can be found in `egli::egl` namespace.
The higher level types are in the root `egli` namespace.

### Lower Level EGL Interface

Lower level interface is very close to raw `ffi`, but with error
handling and unsafety removed (except few special cases).

### Higher Level EGL Interface

EGLI has [RAII](https://en.wikipedia.org/wiki/Resource_Acquisition_Is_Initialization)
wrappers for concepts such as `Surface`, `Display` or `Context`. Such structs
are clearly marked as `RAII` in the documentation, because the user MUST
be aware of resource destruction when these structs go out of scope.

This library does not try to be safe and reference-count the resources.
Instead, the user must manage destruction order manually.

In the following example, the `Display` will be destroyed last, at the end of
scope:

```rust
let display = egli::Display::from_default_display()
                      .expect("failed to get EGL display");
let surface = display.create_window_surface(config, native_window)
                     .expect("failed to create surface");

// at the end of scope the surface will be dropped
// and then the display will be dropped

// the resources will be freed in this exact order
```

If then display and surface are stored in some other struct, care must be taken
to use an order which is reverse of creation:

```rust
let window_info = DisplayAndSurface {
    surface: surface,
    display: display,
};
```

Also, an RC wrapper can be easily written which takes care of these dependencies
as needed by application. This kind of thing is out of scope of this library.

### Using both Higher and Lower Level interfaces

All the `RAII` objects can be created directly from handles,
and all of them have `forget()` method that returns the handle
and disables `RAII` drop function.

In the following example, the display is terminated with lower level
EGL call instead of the end-of-scope drop:

```rust
let display = egli::Display::from_default_display()
                            .expect("failed to get EGL display");

let display_handle = display.forget();

egli::egl::terminate(display_handle) // display is terminated
    .expect("failed to terminate display");

// the display's drop won't run because the forget() was called
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Contains work Copyright 2015 Sean Kerr, Apache License, Version 2.0. Files
under this license can be identified by their headers.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
