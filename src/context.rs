use egl;

/// `[EGL 1.0]` [RAII](https://en.wikipedia.org/wiki/Resource_Acquisition_Is_Initialization) wrapper for
/// EGLContext.
///
/// When dropped, frees up the context with `eglDestroyContext` call.
///
/// Note that the surface would not be immediately freed if it is current to any thread.
/// In such a case, the surface will be freed when it is no longer used.
pub struct Context {
    terminated: bool,
    display_handle: egl::EGLDisplay,
    handle: egl::EGLContext,
}

impl Drop for Context {
    fn drop(&mut self) {
        if !self.terminated {
            trace!("destroy context");
            let _ = egl::destroy_context(self.display_handle, self.handle);
        }
    }
}

impl Into<egl::EGLContext> for Context {
    fn into(self) -> egl::EGLContext {
        self.forget()
    }
}

impl Context {
    /// Create a `Context` from an existing EGL display and context handles.
    pub fn from_handle(display_handle: egl::EGLDisplay,
                       context_handle: egl::EGLSurface)
                       -> Context {
        Context {
            terminated: false,
            display_handle: display_handle,
            handle: context_handle,
        }
    }

    /// Get raw handle.
    pub fn handle(&self) -> egl::EGLContext {
        self.handle
    }

    /// Drops `Context` without cleaning up any resources.
    ///
    /// Returns `EGLContext` handle.
    ///
    /// Alias for `Into<egl::EGLContext>`.
    pub fn forget(mut self) -> egl::EGLContext {
        self.terminated = true;
        self.handle
    }
}
