## EGLI - Easy to use EGL Interface for Rust

Contains two abstraction levels.

Lower level EGL can be found in `egl` namespace. It was implemented by
Sean Kerr, however it does not seem to be maintained anymore, which is
one reason why the lib was copied here. Another reason - dependency on
the old `libc`.

The higher level types are in the root `egli` namespace.

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
