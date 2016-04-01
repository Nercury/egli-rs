// Copyright 2016 The EGLI Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/*!
<a href="https://github.com/Nercury/egli-rs">
    <img style="position: absolute; top: 0; left: 0; border: 0;" src="https://s3.amazonaws.com/github/ribbons/forkme_left_darkblue_121621.png" alt="Fork me on GitHub">
</a>
<style>.sidebar { margin-top: 53px }</style>

# EGLI - Higher-level EGL Interface
*/

extern crate libc;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate log;

pub mod egl;
pub mod ffi;
pub mod error;

mod display;
mod context;
mod window_surface;
mod config_filter;
mod frame_buffer_config;
mod version;

pub use display::{Display, ContextClientVersion};
pub use context::Context;
pub use window_surface::Surface;
pub use config_filter::ConfigFilterRef;
pub use frame_buffer_config::FrameBufferConfigRef;
pub use version::Version;

use std::mem;

/// `[EGL 1.5]` Get supported EGL client version.
///
/// Returns a version or release number.
/// The EGL_VERSION string is laid out as follows:
///
/// `major_version.minor_version space vendor_specific_info`
///
/// Both the major and minor portions of the version number are numeric.
/// Their values must match the major and minor values returned by `Display::initialize`.
#[cfg(feature = "egl_1_5")]
pub fn query_version() -> error::Result<&'static str> {
    let cstr = try!(egl::query_string(egl::EGL_NO_DISPLAY, egl::EGL_VERSION));
    Ok(try!(cstr.to_str()))
}

/// `[EGL 1.0]` Get all supported client extensions.
///
/// Returns a space separated list of supported extensions.
pub fn query_extensions() -> error::Result<&'static str> {
    let cstr = try!(egl::query_string(egl::EGL_NO_DISPLAY, egl::EGL_EXTENSIONS));
    Ok(try!(cstr.to_str()))
}

#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum ColorBufferType {
    Rgb = 0x308E,
    Luminance = 0x308F,
}

impl ColorBufferType {
    pub unsafe fn from_raw(value: egl::EGLint) -> ColorBufferType {
        mem::transmute(value as i32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum ConfigCaveat {
    None = 0x3038,
    Slow = 0x3050,
    NonConformant = 0x3051,
}

impl ConfigCaveat {
    pub unsafe fn from_raw(value: egl::EGLint) -> ConfigCaveat {
        mem::transmute(value as i32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum TransparentType {
    None = 0x3038,
    TransparentRgb = 0x3052,
}

impl TransparentType {
    pub unsafe fn from_raw(value: egl::EGLint) -> TransparentType {
        mem::transmute(value as i32)
    }
}

/// Renderable type mask bits.
pub mod renderable {
    bitflags! {
        pub flags Type: i32 {
            /// EGL_OPENGL_BIT
            const OPENGL       = 0x0008,
            /// EGL_OPENGL_ES_BIT
            const OPENGL_ES    = 0x0001,
            /// EGL_OPENGL_ES2_BIT
            const OPENGL_ES2   = 0x0004,
            /// EGL_OPENGL_ES3_BIT
            const OPENGL_ES3   = 0x00000040,
            /// EGL_OPENVG_BIT
            const OPENVG       = 0x0002,
        }
    }
}

/// Surface type mask bits.
pub mod surface {
    bitflags! {
        pub flags Type: i32 {
            /// EGL_PBUFFER_BIT
            const PBUFFER                  = 0x0001,
            /// EGL_PIXMAP_BIT
            const PIXMAP                   = 0x0002,
            /// EGL_WINDOW_BIT
            const WINDOW                   = 0x0004,
            /// EGL_VG_COLORSPACE_LINEAR_BIT
            const VG_COLORSPACE_LINEAR     = 0x0020,
            /// EGL_VG_ALPHA_FORMAT_PRE_BIT
            const VG_ALPHA_FORMAT_PRE      = 0x0040,
            /// EGL_MULTISAMPLE_RESOLVE_BOX_BIT
            const MULTISAMPLE_RESOLVE_BOX  = 0x0200,
            /// EGL_SWAP_BEHAVIOR_PRESERVED_BIT
            const SWAP_BEHAVIOR_PRESERVED  = 0x0400,
        }
    }
}
