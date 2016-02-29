//! Unsafe FFI API.
//!
//! Use these only if higher-level EGLI or EGL abstrations are not enough.

use libc::c_char;

use egl::{
    EGLBoolean,
    EGLClientBuffer,
    EGLConfig,
    EGLContext,
    EGLDisplay,
    EGLenum,
    EGLint,
    EGLNativeDisplayType,
    EGLNativePixmapType,
    EGLNativeWindowType,
    EGLSurface
};

extern {
    pub fn eglBindAPI(api: EGLenum) -> EGLBoolean;

    pub fn eglBindTexImage(dpy: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> EGLBoolean;

    pub fn eglChooseConfig(dpy: EGLDisplay, attrib_list: *const EGLint,
                           configs: *mut EGLConfig, config_size: EGLint,
                           num_config: *mut EGLint) -> EGLBoolean;

    pub fn eglCopyBuffers(dpy: EGLDisplay, surface: EGLSurface,
                          target: EGLNativePixmapType) -> EGLBoolean;

    pub fn eglCreateContext(dpy: EGLDisplay, config: EGLConfig,
                            share_context: EGLContext,
                            attrib_list: *const EGLint) -> EGLContext;

    pub fn eglCreatePbufferFromClientBuffer(dpy: EGLDisplay, buftype: EGLenum,
                                            buffer: EGLClientBuffer, config: EGLConfig,
                                            attrib_list: *const EGLint) -> EGLSurface;

    pub fn eglCreatePbufferSurface(dpy: EGLDisplay, config: EGLConfig,
                                   attrib_list: *const EGLint) -> EGLSurface;

    pub fn eglCreatePixmapSurface(dpy: EGLDisplay, config: EGLConfig,
                                  pixmap: EGLNativePixmapType,
                                  attrib_list: *const EGLint) -> EGLSurface;

    pub fn eglCreateWindowSurface(dpy: EGLDisplay, config: EGLConfig,
                                  win: EGLNativeWindowType,
                                  attrib_list: *const EGLint) -> EGLSurface;

    pub fn eglDestroyContext(dpy: EGLDisplay, ctx: EGLContext) -> EGLBoolean;

    pub fn eglDestroySurface(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;

    pub fn eglGetConfigAttrib(dpy: EGLDisplay, config: EGLConfig,
                              attribute: EGLint, value: *mut EGLint) -> EGLBoolean;

    pub fn eglGetConfigs(dpy: EGLDisplay, configs: EGLConfig,
                         config_size: EGLint, num_config: *mut EGLint) -> EGLBoolean;

    pub fn eglGetCurrentContext() -> EGLContext;

    pub fn eglGetCurrentDisplay() -> EGLDisplay;

    pub fn eglGetCurrentSurface(readdraw: EGLint) -> EGLSurface;

    pub fn eglGetDisplay(display_id: EGLNativeDisplayType) -> EGLDisplay;

    pub fn eglGetError() -> EGLint;

    pub fn eglGetProcAddress(procname: *const c_char) -> extern "C" fn();

    pub fn eglInitialize(dpy: EGLDisplay, major: *mut EGLint, minor: *mut EGLint) -> EGLBoolean;

    pub fn eglMakeCurrent(dpy: EGLDisplay, draw: EGLSurface,
                          read: EGLSurface, ctx: EGLContext) -> EGLBoolean;

    pub fn eglQueryAPI() -> EGLenum;

    pub fn eglQueryContext(dpy: EGLDisplay, ctx: EGLContext,
                           attribute: EGLint, value: *mut EGLint) -> EGLBoolean;

    pub fn eglQueryString(dpy: EGLDisplay, name: EGLint) -> *const c_char;

    pub fn eglQuerySurface(dpy: EGLDisplay, surface: EGLSurface,
                           attribute: EGLint, value: *mut EGLint) -> EGLBoolean;

    pub fn eglReleaseTexImage(dpy: EGLDisplay, surface: EGLSurface,
                              buffer: EGLint) -> EGLBoolean;

    pub fn eglReleaseThread() -> EGLBoolean;

    pub fn eglSurfaceAttrib(dpy: EGLDisplay, surface: EGLSurface,
                            attribute: EGLint, value: EGLint) -> EGLBoolean;

    pub fn eglSwapBuffers(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;

    pub fn eglSwapInterval(dpy: EGLDisplay, interval: EGLint) -> EGLBoolean;

    pub fn eglTerminate(dpy: EGLDisplay) -> EGLBoolean;

    pub fn eglWaitClient() -> EGLBoolean;

    pub fn eglWaitGL() -> EGLBoolean;

    pub fn eglWaitNative(engine: EGLint) -> EGLBoolean;
}
