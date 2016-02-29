// Copyright 2016 The EGLI Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # EGLI - Higher-level EGL Interface

extern crate libc;
#[macro_use] extern crate bitflags;

pub mod egl;
pub mod ffi;
pub mod error;

mod display;
mod config_filter;
mod frame_buffer_config;
mod version;

pub use display::Display;
pub use config_filter::ConfigFilterRef;
pub use frame_buffer_config::FrameBufferConfigRef;
pub use version::Version;

use std::mem;

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
