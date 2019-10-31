// Copyright 2016 The EGLI Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use egl;
use std::ptr;
use error::Result;
use {Surface, Context, Version, FrameBufferConfigRef, ConfigFilterRef};

pub enum ContextClientVersion {
    OpenGlEs1,
    OpenGlEs2,
}

/// `[EGL 1.0]` [RAII](https://en.wikipedia.org/wiki/Resource_Acquisition_Is_Initialization) wrapper for
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
            let _ = self.make_not_current();
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
    /// `[EGL 1.0]` Create a `Display` from an EGL display connection.
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
            Ok(handle) => {
                Ok(Display {
                    terminated: false,
                    handle: handle,
                })
            }
            Err(e) => Err(e.into()),
        }
    }

    /// `[EGL 1.0]` Creates a `Display` from the default display.
    ///
    /// This is a convenience wrapper that calls `Display::from_display_id` with
    /// `egl::EGL_DEFAULT_DISPLAY` option.
    pub fn from_default_display() -> Result<Display> {
        Display::from_display_id(egl::EGL_DEFAULT_DISPLAY)
    }

    /// `[EGL 1.0]` Initialize this EGL display connection and return EGL version.
    ///
    /// `eglInitialize` initializes the EGL display connection obtained with `eglGetDisplay`.
    /// Initializing an already initialized EGL display connection has no effect besides
    /// returning the version numbers.
    pub fn initialize_and_get_version(&self) -> Result<Version> {
        let (mut major, mut minor) = (0, 0);

        egl::initialize_and_get_version(self.handle, &mut major, &mut minor)?;

        Ok(Version {
            major: major as i32,
            minor: minor as i32,
        })
    }

    /// `[EGL 1.0]` Initialize this EGL display connection.
    ///
    /// `eglInitialize` initializes the EGL display connection obtained with `eglGetDisplay`.
    /// Initializing an already initialized EGL display connection has no effect.
    pub fn initialize(&self) -> Result<()> {

        egl::initialize(self.handle)?;

        Ok(())
    }

    /// `[EGL 1.2]` Query EGL_CLIENT_APIS.
    ///
    /// Returns a string describing which client rendering APIs are supported.
    /// The string contains a space-separate list of API names. The list must
    /// include at least one of OpenGL, OpenGL_ES, or OpenVG.
    /// These strings correspond respectively to values EGL_OPENGL_API, EGL_OPENGL_ES_API, and
    /// EGL_OPENVG_API of the eglBindAPI, api argument.
    pub fn query_client_apis(&self) -> Result<&'static str> {
        let cstr = egl::query_string(self.handle, egl::EGL_CLIENT_APIS)?;
        Ok(cstr.to_str()?)
    }

    /// `[EGL 1.0]` Query EGL_VENDOR.
    ///
    /// The vendor-specific information is optional; if present, its format
    /// and contents are implementation specific.
    pub fn query_vendor(&self) -> Result<&'static str> {
        let cstr = egl::query_string(self.handle, egl::EGL_VENDOR)?;
        Ok(cstr.to_str()?)
    }

    /// `[EGL 1.0]` Get supported EGL version for this display.
    ///
    /// Returns a version or release number.
    /// The EGL_VERSION string is laid out as follows:
    ///
    /// `major_version.minor_version space vendor_specific_info`
    ///
    /// Both the major and minor portions of the version number are numeric.
    /// Their values must match the major and minor values returned by initialize.
    pub fn query_version(&self) -> Result<&'static str> {
        let cstr = egl::query_string(self.handle, egl::EGL_VERSION)?;
        Ok(cstr.to_str()?)
    }

    /// `[EGL 1.0]` Get the set of display extensions supported by this display.
    ///
    /// Returns a space separated list of supported extensions.
    pub fn query_extensions(&self) -> Result<&'static str> {
        let cstr = egl::query_string(self.handle, egl::EGL_EXTENSIONS)?;
        Ok(cstr.to_str()?)
    }

    /// `[EGL 1.0]` Get all possible display configurations.
    ///
    /// Internally, this calls `eglGetConfigs` twice: to get total config count,
    /// and to fill allocated memory with config handles.
    ///
    /// These handles are then wrapped into a new `Vec<FrameBufferConfigRef>`.
    pub fn get_configs(&self) -> Result<Vec<FrameBufferConfigRef>> {
        let count = egl::num_configs(self.handle)? as usize;

        let mut configs: Vec<egl::EGLConfig> = vec![ptr::null_mut(); count];
        let returned_count = egl::get_configs(self.handle, &mut configs)? as usize;

        Ok(configs[..returned_count]
               .iter()
               .map(|c| FrameBufferConfigRef::from_native(self.handle, *c))
               .collect())
    }

    /// `[EGL 1.0]` Creates a new config filter for this display for safe
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
    ///                      .with_red_size(8)
    ///                      .choose_configs();
    /// ```
    pub fn config_filter(&self) -> ConfigFilterRef {
        ConfigFilterRef::from_native(self.handle)
    }

    /// `[EGL 1.0]` Create a new EGL window surface.
    pub fn create_window_surface(&self,
                                 config: FrameBufferConfigRef,
                                 window: egl::EGLNativeWindowType)
                                 -> Result<Surface> {

        let maybe_handle = egl::create_window_surface(self.handle, config.handle(), window);

        Ok(Surface::from_handle(self.handle, maybe_handle?))
    }

    /// `[EGL 1.0]` Create a new EGL rendering context.
    pub fn create_context(&self, config: FrameBufferConfigRef) -> Result<Context> {

        let maybe_handle = egl::create_context(self.handle, config.handle());

        Ok(Context::from_handle(self.handle, maybe_handle?))
    }

    /// `[EGL 1.3]` Create a new EGL rendering context.
    pub fn create_context_with_client_version(&self,
                                              config: FrameBufferConfigRef,
                                              client_version: ContextClientVersion)
                                              -> Result<Context> {

        let attribs = [egl::EGL_CONTEXT_CLIENT_VERSION,
                       match client_version {
                           ContextClientVersion::OpenGlEs1 => 1,
                           ContextClientVersion::OpenGlEs2 => 2,
                       },
                       egl::EGL_NONE];

        let maybe_handle = egl::create_context_with_attribs(self.handle,
                                                            config.handle(),
                                                            ptr::null_mut(),
                                                            &attribs);

        Ok(Context::from_handle(self.handle, maybe_handle?))
    }

    /// `[EGL 1.0]` Attach an EGL rendering context to EGL surfaces.
    pub fn make_current(&self, draw: &Surface, read: &Surface, context: &Context) -> Result<()> {
        egl::make_current(self.handle, draw.handle(), read.handle(), context.handle())?;
        Ok(())
    }

    /// `[EGL 1.0]` Detatch an EGL rendering context from EGL surfaces and contexts.
    pub fn make_not_current(&self) -> Result<()> {
        egl::make_current(self.handle,
                               egl::EGL_NO_SURFACE,
                               egl::EGL_NO_SURFACE,
                               egl::EGL_NO_CONTEXT)?;
        Ok(())
    }

    /// `[EGL 1.0]` Post EGL surface color buffer to a native window.
    pub fn swap_buffers(&self, surface: &Surface) -> Result<()> {
        egl::swap_buffers(self.handle, surface.handle())?;
        Ok(())
    }

    /// Run an action with inner handle as parameter.
    pub fn with_handle<F, R>(&self, action: F) -> R
        where F: FnOnce(egl::EGLDisplay) -> R
    {
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
