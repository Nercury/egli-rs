// Copyright 2016 The EGLI Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use egl;
use error::Result;

/// `[EGL 1.0]` [RAII](https://en.wikipedia.org/wiki/Resource_Acquisition_Is_Initialization) wrapper for
/// EGLSurface.
///
/// When dropped, frees up the surface with `eglDestroySurface` call.
///
/// Note that the surface would not be immediately freed if it is current to any thread.
/// In such a case, the surface will be freed when it is no longer used.
pub struct Surface {
    terminated: bool,
    display_handle: egl::EGLDisplay,
    handle: egl::EGLSurface,
}

impl Drop for Surface {
    fn drop(&mut self) {
        if !self.terminated {
            let _ = egl::destroy_surface(self.display_handle, self.handle);
        }
    }
}

impl Into<egl::EGLSurface> for Surface {
    fn into(self) -> egl::EGLSurface {
        self.forget()
    }
}

impl Surface {
    /// Create a `Surface` from an existing EGL display and surface handles.
    pub fn from_handle(
        display_handle: egl::EGLDisplay,
        surface_handle: egl::EGLSurface,
    ) -> Surface {
        Surface {
            terminated: false,
            display_handle: display_handle,
            handle: surface_handle,
        }
    }

    /// Get raw handle.
    pub fn handle(&self) -> egl::EGLSurface {
        self.handle
    }

    /// [EGL 1.0] Returns the width of the surface in pixels.
    ///
    /// Result of `eglQuerySurface` with `EGL_WIDTH` parameter.
    pub fn query_width(&self) -> Result<i32> {
        let mut value: egl::EGLint = 0;
        egl::query_surface(self.display_handle, self.handle, egl::EGL_WIDTH, &mut value)?;
        Ok(value as i32)
    }

    /// [EGL 1.0] Returns the height of the surface in pixels.
    ///
    /// Result of `eglQuerySurface` with `EGL_HEIGHT` parameter.
    pub fn query_height(&self) -> Result<i32> {
        let mut value: egl::EGLint = 0;
        egl::query_surface(
            self.display_handle,
            self.handle,
            egl::EGL_HEIGHT,
            &mut value,
        )?;
        Ok(value as i32)
    }

    /// Drops `Surface` without cleaning up any resources.
    ///
    /// Returns `EGLSurface` handle.
    ///
    /// Alias for `Into<egl::EGLSurface>`.
    pub fn forget(mut self) -> egl::EGLSurface {
        self.terminated = true;
        self.handle
    }
}
