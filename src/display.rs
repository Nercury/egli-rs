use egl;
use std::ptr;
use error::Result;
use {
    Version,
    FrameBufferConfigRef,
    ConfigFilterRef
};

/// [RAII](https://en.wikipedia.org/wiki/Resource_Acquisition_Is_Initialization) wrapper for
/// EGLDisplay.
///
/// When dropped, frees up any associated surface and context using
///
/// ```ignore
/// eglMakeCurrent(.., NO_SURFACE, NO_SURFACE, NO_CONTEXT);
/// ```
///
/// call. Followed by `eglTerminate`.
pub struct Display {
    terminated: bool,
    handle: egl::EGLDisplay,
}

impl Drop for Display {
    fn drop(&mut self) {
        if !self.terminated {
            // If you are reading this, you may be wondering if the drop panics. It does not.
            // Ignoring errors here might be sub-optimal for some API uses.
            // In that case, use EGL directly, or handle termination by getting handle from
            // `forget` method.
            let _ = egl::make_current(self.handle, egl::EGL_NO_SURFACE, egl::EGL_NO_SURFACE, egl::EGL_NO_CONTEXT);
            let _ = egl::terminate(self.handle);
        }
    }
}

impl Into<egl::EGLDisplay> for Display {
    fn into(self) -> egl::EGLDisplay {
        self.forget()
    }
}

impl Display {
    /// Create a `Display` from an EGL display connection.
    ///
    /// On success, returns a `Display` value that will clean up resources when terminated.
    ///
    /// If no display connection matching `display_id` is available, EGL_NO_DISPLAY is
    /// returned. No error is generated.
    ///
    /// ## display_id
    ///
    /// Specifies the display to connect to. `egl::EGL_DEFAULT_DISPLAY` indicates the
    /// default display.
    pub fn from_display_id(display_id: egl::EGLNativeDisplayType) -> Result<Display> {
        match egl::get_display(display_id) {
            Ok(handle) => Ok(Display {
                terminated: false,
                handle: handle,
            }),
            Err(e) => Err(e.into()),
        }
    }

    /// Creates a `Display` from the default display.
    ///
    /// This is a convenience wrapper that calls `Display::from_display_id` with
    /// `egl::EGL_DEFAULT_DISPLAY` option.
    pub fn from_default_display() -> Result<Display> {
        Display::from_display_id(egl::EGL_DEFAULT_DISPLAY)
    }

    /// Initialize this EGL display connection and return EGL version.
    ///
    /// `eglInitialize` initializes the EGL display connection obtained with `eglGetDisplay`.
    /// Initializing an already initialized EGL display connection has no effect besides
    /// returning the version numbers.
    pub fn initialize_and_get_version(&self) -> Result<Version> {
        let (mut major, mut minor) = (0, 0);
        try!(egl::initialize_and_get_version(self.handle, &mut major, &mut minor));

        Ok(Version {
            major: major as i32,
            minor: minor as i32,
        })
    }

    /// Initialize this EGL display connection.
    ///
    /// `eglInitialize` initializes the EGL display connection obtained with `eglGetDisplay`.
    /// Initializing an already initialized EGL display connection has no effect.
    pub fn initialize(&self) -> Result<()> {
        try!(egl::initialize(self.handle));
        Ok(())
    }

    /// Get all possible display configurations.
    ///
    /// Internally, this calls `eglGetConfigs` twice: to get total config count,
    /// and to fill allocated memory with config handles.
    ///
    /// These handles are then wrapped into a new `Vec<FrameBufferConfigRef>`.
    pub fn get_configs(&self) -> Result<Vec<FrameBufferConfigRef>> {
        let count = try!(egl::num_configs(self.handle)) as usize;

        let mut configs: Vec<egl::EGLConfig> = vec![ptr::null_mut(); count];
        let returned_count = try!(egl::get_configs(self.handle, &mut configs)) as usize;

        Ok(configs[..returned_count].iter()
            .map(|c| FrameBufferConfigRef::from_native(self.handle, *c))
            .collect())
    }

    /// Creates a new config filter for this display for safe
    /// invocation of `eglChooseConfig`.
    ///
    /// See documentation of `ConfigFilterRef` for the list of all available filter
    /// methods.
    ///
    /// ## Example
    ///
    /// ```
    /// use egli::Display;
    ///
    /// let display = Display::from_default_display()
    ///                      .expect("failed to get default display");
    /// let configs = display.config_filter()
    ///                      .with_alpha_mask_size(8)
    ///                      .choose_configs()
    ///                      .expect("failed to get display configs");
    /// ```
    pub fn config_filter(&self) -> ConfigFilterRef {
        ConfigFilterRef::from_native(self.handle)
    }

    /// Run an action with inner handle as parameter.
    pub fn with_handle<F, R>(&self, action: F) -> R where F: FnOnce(egl::EGLDisplay) -> R {
        action(self.handle)
    }

    /// Drops `Display` without cleaning up any resources.
    ///
    /// Returns `EGLDisplay` handle.
    ///
    /// Alias for `Into<egl::EGLDisplay>`.
    pub fn forget(mut self) -> egl::EGLDisplay {
        self.terminated = true;
        self.handle
    }
}
