## EGLI - EGL Interface for Rust

Contains two abstraction levels.

Lower level EGL can be found in `egl` namespace.
The higher level types are in the root `egli` namespace.

## Lower Level EGL Interface

Lower level interface is very close to raw `ffi`, but with error
handling and unsafety removed (except few special cases).

## Higher Level EGL Interface

EGLI has [RAII](https://en.wikipedia.org/wiki/Resource_Acquisition_Is_Initialization)
wrappers for concepts such as `Surface`, `Display` or `Context`. Such structs
are clearly marked as `RAII` in the documentation, because the user MUST
be aware of resource destruction when these structs go out of scope.

This library does not try to be safe and reference-count the resources.
Instead, the user must manage destruction order manually.

In the following example, the `Display` will be destroyed last, at the end of
scope:

```rust
let display = Display::from_default_display()
                      .expect("failed to get EGL display");
let surface = display.create_window_surface(config, native_window)
                     .expect("failed to create surface");

// at the end of scope the surface will be droped
// and then the display will be droped

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

## Using both Higher and Lower Level interfaces

All the `RAII` objects can be created directly from handles,
and all of them have `forget()` method that returns the handle
and disables `RAII` drop function.

In the following example, the display is terminated with lower level
EGL call instead of the end-of-scope drop:

```rust
let display = egli::Display::from_default_display()
                            .expect("failed to get EGL display");

let display_handle = display.forget();

egl::terminate(display_handle) // display is terminated
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
