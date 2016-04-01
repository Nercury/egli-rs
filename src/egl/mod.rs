// +-----------------------------------------------------------------------------------------------+
// | Copyright 2015 Sean Kerr                                                                      |
// |                                                                                               |
// | Licensed under the Apache License, Version 2.0 (the "License");                               |
// | you may not use this file except in compliance with the License.                              |
// | You may obtain a copy of the License Author                                                   |
// |                                                                                               |
// |  http://www.apache.org/licenses/LICENSE-2.0                                                   |
// |                                                                                               |
// | Unless required by applicable law or agreed to in writing, software                           |
// | distributed under the License is distributed on an "AS IS" BASIS,                             |
// | WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.                      |
// | See the License for the specific language governing permissions and                           |
// | limitations under the License.                                                                |
// +-----------------------------------------------------------------------------------------------+
// | Author: Sean Kerr <sean@code-box.org>                                                         |
// +-----------------------------------------------------------------------------------------------+

//! C-style lower level EGL wrapper with unsafety removed.
//!
//! Use these only if higher-level EGLI abstration is not enough, and you need to manage
//! resource clanup manually.

#![allow(dead_code)]
#![allow(non_upper_case_globals)]

// -------------------------------------------------------------------------------------------------
// DEPENDENCIES
// -------------------------------------------------------------------------------------------------

mod khronos;

use std::mem;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;
use ffi;
use error::{EglCallError, EglCallResult};

use libc::{c_uint, c_void, int32_t};

// -------------------------------------------------------------------------------------------------
// LINKING
// -------------------------------------------------------------------------------------------------

#[link(name = "EGL")]
extern "C" {}

// -------------------------------------------------------------------------------------------------
// GLOBAL TYPES
// -------------------------------------------------------------------------------------------------

pub type EGLBoolean = c_uint;
pub type EGLClientBuffer = *mut c_void;
pub type EGLConfig = *mut c_void;
pub type EGLContext = *mut c_void;
pub type EGLDisplay = *mut c_void;
pub type EGLenum = c_uint;
pub type EGLint = khronos::khronos_int32_t;
pub type EGLNativeDisplayType = *mut c_void;
pub type EGLSurface = *mut c_void;
// EGL 1.5
#[cfg(feature = "egl_1_5")]
pub type EGLSync = *mut c_void;
#[cfg(feature = "egl_1_5")]
pub type EGLAttrib = khronos::khronos_intptr_t;
#[cfg(feature = "egl_1_5")]
pub type EGLTime = khronos::khronos_utime_nanoseconds_t;
#[cfg(feature = "egl_1_5")]
pub type EGLImage = *mut c_void;

// -------------------------------------------------------------------------------------------------
// ANDROID TYPES
// -------------------------------------------------------------------------------------------------

#[repr(C)]
#[cfg(android)]
struct android_native_window_t;

#[repr(C)]
#[cfg(android)]
struct egl_native_pixmap_t;

#[cfg(android)]
pub type EGLNativePixmapType = *mut egl_native_pixmap_t;

#[cfg(android)]
pub type EGLNativeWindowType = *mut android_native_window_t;

// -------------------------------------------------------------------------------------------------
// NON-ANDROID TYPES
// -------------------------------------------------------------------------------------------------

#[cfg(not(android))]
pub type EGLNativePixmapType = *mut c_void;

#[cfg(not(android))]
pub type EGLNativeWindowType = *mut c_void;

// -------------------------------------------------------------------------------------------------
// CONSTANTS
// -------------------------------------------------------------------------------------------------

// EGL aliases
pub const EGL_FALSE: EGLBoolean = 0;
pub const EGL_TRUE: EGLBoolean = 1;

// out-of-band handle values
pub const EGL_DEFAULT_DISPLAY: EGLNativeDisplayType = 0 as *mut c_void;
pub const EGL_NO_CONTEXT: EGLContext = 0 as *mut c_void;
pub const EGL_NO_DISPLAY: EGLDisplay = 0 as *mut c_void;
pub const EGL_NO_SURFACE: EGLSurface = 0 as *mut c_void;

// out-of-band attribute value
pub const EGL_DONT_CARE: EGLint = -1;

// errors / GetError return values
pub const EGL_SUCCESS: EGLint = 0x3000;
pub const EGL_NOT_INITIALIZED: EGLint = 0x3001;
pub const EGL_BAD_ACCESS: EGLint = 0x3002;
pub const EGL_BAD_ALLOC: EGLint = 0x3003;
pub const EGL_BAD_ATTRIBUTE: EGLint = 0x3004;
pub const EGL_BAD_CONFIG: EGLint = 0x3005;
pub const EGL_BAD_CONTEXT: EGLint = 0x3006;
pub const EGL_BAD_CURRENT_SURFACE: EGLint = 0x3007;
pub const EGL_BAD_DISPLAY: EGLint = 0x3008;
pub const EGL_BAD_MATCH: EGLint = 0x3009;
pub const EGL_BAD_NATIVE_PIXMAP: EGLint = 0x300A;
pub const EGL_BAD_NATIVE_WINDOW: EGLint = 0x300B;
pub const EGL_BAD_PARAMETER: EGLint = 0x300C;
pub const EGL_BAD_SURFACE: EGLint = 0x300D;
pub const EGL_CONTEXT_LOST: EGLint = 0x300E;  // EGL 1.1 - IMG_power_management

// config attributes
pub const EGL_BUFFER_SIZE: EGLint = 0x3020;
pub const EGL_ALPHA_SIZE: EGLint = 0x3021;
pub const EGL_BLUE_SIZE: EGLint = 0x3022;
pub const EGL_GREEN_SIZE: EGLint = 0x3023;
pub const EGL_RED_SIZE: EGLint = 0x3024;
pub const EGL_DEPTH_SIZE: EGLint = 0x3025;
pub const EGL_STENCIL_SIZE: EGLint = 0x3026;
pub const EGL_CONFIG_CAVEAT: EGLint = 0x3027;
pub const EGL_CONFIG_ID: EGLint = 0x3028;
pub const EGL_LEVEL: EGLint = 0x3029;
pub const EGL_MAX_PBUFFER_HEIGHT: EGLint = 0x302A;
pub const EGL_MAX_PBUFFER_PIXELS: EGLint = 0x302B;
pub const EGL_MAX_PBUFFER_WIDTH: EGLint = 0x302C;
pub const EGL_NATIVE_RENDERABLE: EGLint = 0x302D;
pub const EGL_NATIVE_VISUAL_ID: EGLint = 0x302E;
pub const EGL_NATIVE_VISUAL_TYPE: EGLint = 0x302F;
pub const EGL_SAMPLES: EGLint = 0x3031;
pub const EGL_SAMPLE_BUFFERS: EGLint = 0x3032;
pub const EGL_SURFACE_TYPE: EGLint = 0x3033;
pub const EGL_TRANSPARENT_TYPE: EGLint = 0x3034;
pub const EGL_TRANSPARENT_BLUE_VALUE: EGLint = 0x3035;
pub const EGL_TRANSPARENT_GREEN_VALUE: EGLint = 0x3036;
pub const EGL_TRANSPARENT_RED_VALUE: EGLint = 0x3037;
pub const EGL_NONE: EGLint = 0x3038; // attrib list terminator
pub const EGL_BIND_TO_TEXTURE_RGB: EGLint = 0x3039;
pub const EGL_BIND_TO_TEXTURE_RGBA: EGLint = 0x303A;
pub const EGL_MIN_SWAP_INTERVAL: EGLint = 0x303B;
pub const EGL_MAX_SWAP_INTERVAL: EGLint = 0x303C;
pub const EGL_LUMINANCE_SIZE: EGLint = 0x303D;
pub const EGL_ALPHA_MASK_SIZE: EGLint = 0x303E;
pub const EGL_COLOR_BUFFER_TYPE: EGLint = 0x303F;
pub const EGL_RENDERABLE_TYPE: EGLint = 0x3040;
pub const EGL_MATCH_NATIVE_PIXMAP: EGLint = 0x3041;  // psseudo-attribute (not queryable)
pub const EGL_CONFORMANT: EGLint = 0x3042;

// config attribute values
pub const EGL_SLOW_CONFIG: EGLint = 0x3050;  // CONFIG_CAVEAT value
pub const EGL_NON_CONFORMANT_CONFIG: EGLint = 0x3051;  // CONFIG_CAVEAT value
pub const EGL_TRANSPARENT_RGB: EGLint = 0x3052;  // TRANSPARENT_TYPE value
pub const EGL_RGB_BUFFER: EGLint = 0x308E;  // COLOR_BUFFER_TYPE value
pub const EGL_LUMINANCE_BUFFER: EGLint = 0x308F;  // COLOR_BUFFER_TYPE value

// more config attribute values, for TEXTURE_FORMAT
pub const EGL_NO_TEXTURE: EGLint = 0x305C;
pub const EGL_TEXTURE_RGB: EGLint = 0x305D;
pub const EGL_TEXTURE_RGBA: EGLint = 0x305E;
pub const EGL_TEXTURE_2D: EGLint = 0x305F;

// QueryString targets
pub const EGL_VENDOR: EGLint = 0x3053;
pub const EGL_VERSION: EGLint = 0x3054;
pub const EGL_EXTENSIONS: EGLint = 0x3055;
pub const EGL_CLIENT_APIS: EGLint = 0x308D;

// QuerySurface / SurfaceAttrib / CreatePbufferSurface targets
pub const EGL_HEIGHT: EGLint = 0x3056;
pub const EGL_WIDTH: EGLint = 0x3057;
pub const EGL_LARGEST_PBUFFER: EGLint = 0x3058;
pub const EGL_TEXTURE_FORMAT: EGLint = 0x3080;
pub const EGL_TEXTURE_TARGET: EGLint = 0x3081;
pub const EGL_MIPMAP_TEXTURE: EGLint = 0x3082;
pub const EGL_MIPMAP_LEVEL: EGLint = 0x3083;
pub const EGL_RENDER_BUFFER: EGLint = 0x3086;
pub const EGL_VG_COLORSPACE: EGLint = 0x3087;
pub const EGL_VG_ALPHA_FORMAT: EGLint = 0x3088;
pub const EGL_HORIZONTAL_RESOLUTION: EGLint = 0x3090;
pub const EGL_VERTICAL_RESOLUTION: EGLint = 0x3091;
pub const EGL_PIXEL_ASPECT_RATIO: EGLint = 0x3092;
pub const EGL_SWAP_BEHAVIOR: EGLint = 0x3093;
pub const EGL_MULTISAMPLE_RESOLVE: EGLint = 0x3099;

// RENDER_BUFFER values / BindTexImage / ReleaseTexImage buffer targets
pub const EGL_BACK_BUFFER: EGLint = 0x3084;
pub const EGL_SINGLE_BUFFER: EGLint = 0x3085;

// OpenVG color spaces */
pub const EGL_VG_COLORSPACE_sRGB: EGLint = 0x3089;  // VG_COLORSPACE value
pub const EGL_VG_COLORSPACE_LINEAR: EGLint = 0x308A;  // VG_COLORSPACE value

// OpenVG alpha formats
pub const EGL_VG_ALPHA_FORMAT_NONPRE: EGLint = 0x308B; // ALPHA_FORMAT value
pub const EGL_VG_ALPHA_FORMAT_PRE: EGLint = 0x308C; // ALPHA_FORMAT value

// constant scale factor by which fractional display resolutions & aspect ratio are scaled when
// queried as integer values
pub const EGL_DISPLAY_SCALING: EGLint = 10000;

// unknown display resolution/aspect ratio
pub const EGL_UNKNOWN: EGLint = -1;

// back buffer swap behaviors
pub const EGL_BUFFER_PRESERVED: EGLint = 0x3094; // SWAP_BEHAVIOR value
pub const EGL_BUFFER_DESTROYED: EGLint = 0x3095; // SWAP_BEHAVIOR value

// CreatePbufferFromClientBuffer buffer types
pub const EGL_OPENVG_IMAGE: EGLint = 0x3096;

// QueryContext targets
pub const EGL_CONTEXT_CLIENT_TYPE: EGLint = 0x3097;

// CreateContext attributes
pub const EGL_CONTEXT_CLIENT_VERSION: EGLint = 0x3098;

// multisample resolution behaviors
pub const EGL_MULTISAMPLE_RESOLVE_DEFAULT: EGLint = 0x309A; // MULTISAMPLE_RESOLVE value
pub const EGL_MULTISAMPLE_RESOLVE_BOX: EGLint = 0x309B; // MULTISAMPLE_RESOLVE value

// BindAPI/QueryAPI targets
pub const EGL_OPENGL_ES_API: EGLenum = 0x30A0;
pub const EGL_OPENVG_API: EGLenum = 0x30A1;
pub const EGL_OPENGL_API: EGLenum = 0x30A2;

// GetCurrentSurface targets
pub const EGL_DRAW: EGLint = 0x3059;
pub const EGL_READ: EGLint = 0x305A;

// WaitNative engines
pub const EGL_CORE_NATIVE_ENGINE: EGLint = 0x305B;

// EGL 1.2 tokens renamed for consistency in EGL 1.3
pub const EGL_COLORSPACE: EGLint = EGL_VG_COLORSPACE;
pub const EGL_ALPHA_FORMAT: EGLint = EGL_VG_ALPHA_FORMAT;
pub const EGL_COLORSPACE_sRGB: EGLint = EGL_VG_COLORSPACE_sRGB;
pub const EGL_COLORSPACE_LINEAR: EGLint = EGL_VG_COLORSPACE_LINEAR;
pub const EGL_ALPHA_FORMAT_NONPRE: EGLint = EGL_VG_ALPHA_FORMAT_NONPRE;
pub const EGL_ALPHA_FORMAT_PRE: EGLint = EGL_VG_ALPHA_FORMAT_PRE;

// EGL 1.5
#[cfg(feature = "egl_1_5")]
pub const EGL_CONTEXT_MAJOR_VERSION: EGLint = 0x3098;
#[cfg(feature = "egl_1_5")]
pub const EGL_CONTEXT_MINOR_VERSION: EGLint = 0x30FB;
#[cfg(feature = "egl_1_5")]
pub const EGL_CONTEXT_OPENGL_PROFILE_MASK: EGLint = 0x30FD;
#[cfg(feature = "egl_1_5")]
pub const EGL_CONTEXT_OPENGL_RESET_NOTIFICATION_STRATEGY: EGLint = 0x31BD;
#[cfg(feature = "egl_1_5")]
pub const EGL_NO_RESET_NOTIFICATION: EGLint = 0x31BE;
#[cfg(feature = "egl_1_5")]
pub const EGL_LOSE_CONTEXT_ON_RESET: EGLint = 0x31BF;
#[cfg(feature = "egl_1_5")]
pub const EGL_CONTEXT_OPENGL_CORE_PROFILE_BIT: EGLint = 0x00000001;
#[cfg(feature = "egl_1_5")]
pub const EGL_CONTEXT_OPENGL_COMPATIBILITY_PROFILE_BIT: EGLint = 0x00000002;
#[cfg(feature = "egl_1_5")]
pub const EGL_CONTEXT_OPENGL_DEBUG: EGLint = 0x31B0;
#[cfg(feature = "egl_1_5")]
pub const EGL_CONTEXT_OPENGL_FORWARD_COMPATIBLE: EGLint = 0x31B1;
#[cfg(feature = "egl_1_5")]
pub const EGL_CONTEXT_OPENGL_ROBUST_ACCESS: EGLint = 0x31B2;
#[cfg(feature = "egl_1_5")]
pub const EGL_OPENGL_ES3_BIT: EGLint = 0x00000040;
#[cfg(feature = "egl_1_5")]
pub const EGL_CL_EVENT_HANDLE: EGLint = 0x309C;
#[cfg(feature = "egl_1_5")]
pub const EGL_SYNC_CL_EVENT: EGLint = 0x30FE;
#[cfg(feature = "egl_1_5")]
pub const EGL_SYNC_CL_EVENT_COMPLETE: EGLint = 0x30FF;
#[cfg(feature = "egl_1_5")]
pub const EGL_SYNC_PRIOR_COMMANDS_COMPLETE: EGLint = 0x30F0;
#[cfg(feature = "egl_1_5")]
pub const EGL_SYNC_TYPE: EGLint = 0x30F7;
#[cfg(feature = "egl_1_5")]
pub const EGL_SYNC_STATUS: EGLint = 0x30F1;
#[cfg(feature = "egl_1_5")]
pub const EGL_SYNC_CONDITION: EGLint = 0x30F8;
#[cfg(feature = "egl_1_5")]
pub const EGL_SIGNALED: EGLint = 0x30F2;
#[cfg(feature = "egl_1_5")]
pub const EGL_UNSIGNALED: EGLint = 0x30F3;
#[cfg(feature = "egl_1_5")]
pub const EGL_SYNC_FLUSH_COMMANDS_BIT: EGLint = 0x0001;
#[cfg(feature = "egl_1_5")]
pub const EGL_FOREVER: u64 = 0xFFFFFFFFFFFFFFFF;
#[cfg(feature = "egl_1_5")]
pub const EGL_TIMEOUT_EXPIRED: EGLint = 0x30F5;
#[cfg(feature = "egl_1_5")]
pub const EGL_CONDITION_SATISFIED: EGLint = 0x30F6;
#[cfg(feature = "egl_1_5")]
pub const EGL_NO_SYNC: EGLSync = 0 as EGLSync;
#[cfg(feature = "egl_1_5")]
pub const EGL_SYNC_FENCE: EGLint = 0x30F9;
#[cfg(feature = "egl_1_5")]
pub const EGL_GL_COLORSPACE: EGLint = 0x309D;
#[cfg(feature = "egl_1_5")]
pub const EGL_GL_COLORSPACE_SRGB: EGLint = 0x3089;
#[cfg(feature = "egl_1_5")]
pub const EGL_GL_COLORSPACE_LINEAR: EGLint = 0x308A;
#[cfg(feature = "egl_1_5")]
pub const EGL_GL_RENDERBUFFER: EGLint = 0x30B9;
#[cfg(feature = "egl_1_5")]
pub const EGL_GL_TEXTURE_2D: EGLint = 0x30B1;
#[cfg(feature = "egl_1_5")]
pub const EGL_GL_TEXTURE_LEVEL: EGLint = 0x30BC;
#[cfg(feature = "egl_1_5")]
pub const EGL_GL_TEXTURE_3D: EGLint = 0x30B2;
#[cfg(feature = "egl_1_5")]
pub const EGL_GL_TEXTURE_ZOFFSET: EGLint = 0x30BD;
#[cfg(feature = "egl_1_5")]
pub const EGL_GL_TEXTURE_CUBE_MAP_POSITIVE_X: EGLint = 0x30B3;
#[cfg(feature = "egl_1_5")]
pub const EGL_GL_TEXTURE_CUBE_MAP_NEGATIVE_X: EGLint = 0x30B4;
#[cfg(feature = "egl_1_5")]
pub const EGL_GL_TEXTURE_CUBE_MAP_POSITIVE_Y: EGLint = 0x30B5;
#[cfg(feature = "egl_1_5")]
pub const EGL_GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: EGLint = 0x30B6;
#[cfg(feature = "egl_1_5")]
pub const EGL_GL_TEXTURE_CUBE_MAP_POSITIVE_Z: EGLint = 0x30B7;
#[cfg(feature = "egl_1_5")]
pub const EGL_GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: EGLint = 0x30B8;
#[cfg(feature = "egl_1_5")]
pub const EGL_IMAGE_PRESERVED: EGLint = 0x30D2;
#[cfg(feature = "egl_1_5")]
pub const EGL_NO_IMAGE: EGLImage = 0 as EGLImage;

// -------------------------------------------------------------------------------------------------
// FUNCTIONS
// -------------------------------------------------------------------------------------------------

/// `[EGL 1.2]` Set the current rendering API.
///
/// ## api
///
/// Specifies the client API to bind, one of EGL_OPENGL_API, EGL_OPENGL_ES_API, or EGL_OPENVG_API.
pub fn bind_api(api: EGLenum) -> EglCallResult<()> {
    if unsafe { ffi::eglBindAPI(api) } == EGL_FALSE {
        return Err(EglCallError::BindAPI);
    }
    Ok(())
}

/// `[EGL 1.1]` Defines a two-dimensional texture image.
pub fn bind_tex_image(display: EGLDisplay,
                      surface: EGLSurface,
                      buffer: EGLint)
                      -> EglCallResult<()> {
    if unsafe { ffi::eglBindTexImage(display, surface, buffer) } != EGL_TRUE {
        return Err(EglCallError::BindTexImage);
    }
    Ok(())
}

/// `[EGL 1.0]` Return the total number of display configs that match specified attributes.
///
/// Calls `eglChooseConfig` internally.
///
/// On failure returns `None`.
pub fn num_filtered_configs(display: EGLDisplay, attrib_list: &[EGLint]) -> EglCallResult<i32> {
    let mut count: int32_t = 0;
    if unsafe {
        ffi::eglChooseConfig(display,
                             attrib_list.as_ptr(),
                             ptr::null_mut(),
                             0,
                             &mut count)
    } != EGL_TRUE {
        return Err(EglCallError::ChooseConfig);
    }
    Ok(count as i32)
}

/// `[EGL 1.0]` Return a list of EGL frame buffer configurations that match specified attributes.
///
/// Calls `eglChooseConfig` internally.
///
/// Returns the number of configs written, `None` on failure.
pub fn get_filtered_configs(display: EGLDisplay,
                            attrib_list: &[EGLint],
                            configs: &mut [EGLConfig])
                            -> EglCallResult<i32> {
    let mut count: int32_t = 0;
    if unsafe {
        ffi::eglChooseConfig(display,
                             attrib_list.as_ptr(),
                             mem::transmute(configs.as_mut_ptr()),
                             configs.len() as int32_t,
                             &mut count)
    } != EGL_TRUE {
        return Err(EglCallError::ChooseConfig);
    }
    Ok(count as i32)
}

/// `[EGL 1.0]` Copy EGL surface color buffer to a native pixmap.
pub fn copy_buffers(display: EGLDisplay,
                    surface: EGLSurface,
                    target: EGLNativePixmapType)
                    -> EglCallResult<()> {
    if unsafe { ffi::eglCopyBuffers(display, surface, target) } != EGL_TRUE {
        return Err(EglCallError::CopyBuffers);
    }
    Ok(())
}

/// `[EGL 1.0]` Create a new EGL rendering context.
pub fn create_context(display: EGLDisplay, config: EGLConfig) -> EglCallResult<EGLContext> {
    unsafe {
        let context = ffi::eglCreateContext(display, config, ptr::null_mut(), ptr::null());

        if !context.is_null() {
            Ok(context)
        } else {
            Err(EglCallError::CreateContext)
        }
    }
}

/// `[EGL 1.0]` Create a new EGL rendering context.
pub fn create_context_with_attribs(display: EGLDisplay,
                                   config: EGLConfig,
                                   share_context: EGLContext,
                                   attrib_list: &[EGLint])
                                   -> EglCallResult<EGLContext> {
    unsafe {
        let context = ffi::eglCreateContext(display, config, share_context, attrib_list.as_ptr());

        if !context.is_null() {
            Ok(context)
        } else {
            Err(EglCallError::CreateContext)
        }
    }
}

/// `[EGL 1.2]` Create a new EGL pixel buffer surface bound to an OpenVG image.
pub fn create_pbuffer_from_client_buffer(display: EGLDisplay,
                                         buffer_type: EGLenum,
                                         buffer: EGLClientBuffer,
                                         config: EGLConfig,
                                         attrib_list: &[EGLint])
                                         -> EglCallResult<EGLSurface> {
    unsafe {
        let attribs = if attrib_list.len() > 0 {
            attrib_list.as_ptr()
        } else {
            ptr::null()
        };

        let surface = ffi::eglCreatePbufferFromClientBuffer(display,
                                                            buffer_type,
                                                            buffer,
                                                            config,
                                                            attribs);

        if !surface.is_null() {
            Ok(surface)
        } else {
            Err(EglCallError::CreatePbufferFromClientBuffer)
        }
    }
}

/// `[EGL 1.0]` Create a new EGL pixel buffer surface.
pub fn create_pbuffer_surface(display: EGLDisplay,
                              config: EGLConfig,
                              attrib_list: &[EGLint])
                              -> EglCallResult<EGLSurface> {
    unsafe {
        let attribs = if attrib_list.len() > 0 {
            attrib_list.as_ptr()
        } else {
            ptr::null()
        };

        let surface = ffi::eglCreatePbufferSurface(display, config, attribs);

        if !surface.is_null() {
            Ok(surface)
        } else {
            Err(EglCallError::CreatePbufferSurface)
        }
    }
}

/// `[EGL 1.0]` Create a new EGL pixmap surface.
pub fn create_pixmap_surface(display: EGLDisplay,
                             config: EGLConfig,
                             pixmap: EGLNativePixmapType,
                             attrib_list: &[EGLint])
                             -> EglCallResult<EGLSurface> {
    unsafe {
        let attribs = if attrib_list.len() > 0 {
            attrib_list.as_ptr()
        } else {
            ptr::null()
        };

        let surface = ffi::eglCreatePixmapSurface(display, config, pixmap, attribs);

        if !surface.is_null() {
            Ok(surface)
        } else {
            Err(EglCallError::CreatePixmapSurface)
        }
    }
}

/// `[EGL 1.0]` Create a new EGL window surface.
pub fn create_window_surface(display: EGLDisplay,
                             config: EGLConfig,
                             window: EGLNativeWindowType)
                             -> EglCallResult<EGLSurface> {
    unsafe {
        let surface = ffi::eglCreateWindowSurface(display, config, window, ptr::null());

        if !surface.is_null() {
            Ok(surface)
        } else {
            Err(EglCallError::CreateWindowSurface)
        }
    }
}

/// `[EGL 1.0]` Create a new EGL window surface.
pub fn create_window_surface_with_attribs(display: EGLDisplay,
                                          config: EGLConfig,
                                          window: EGLNativeWindowType,
                                          attrib_list: &[EGLint])
                                          -> EglCallResult<EGLSurface> {
    unsafe {
        let surface = ffi::eglCreateWindowSurface(display, config, window, attrib_list.as_ptr());

        if !surface.is_null() {
            Ok(surface)
        } else {
            Err(EglCallError::CreateWindowSurface)
        }
    }
}

/// `[EGL 1.5]` Create a new EGL window surface.
#[cfg(feature = "egl_1_5")]
pub fn create_platform_window_surface(display: EGLDisplay,
                                      config: EGLConfig,
                                      native_window: *mut c_void,
                                      attrib_list: &[EGLAttrib])
                                      -> EglCallResult<EGLSurface> {
    unsafe {
        let attribs = if attrib_list.len() > 0 {
            attrib_list.as_ptr()
        } else {
            ptr::null()
        };

        let surface = ffi::eglCreatePlatformWindowSurface(display, config, native_window, attribs);

        if !surface.is_null() {
            Ok(surface)
        } else {
            Err(EglCallError::CreatePlatformWindowSurface)
        }
    }
}

/// `[EGL 1.0]` Destroy an EGL rendering context.
pub fn destroy_context(display: EGLDisplay, ctx: EGLContext) -> EglCallResult<()> {
    if unsafe { ffi::eglDestroyContext(display, ctx) } != EGL_TRUE {
        return Err(EglCallError::DestroyContext);
    }
    Ok(())
}

/// `[EGL 1.0]` Destroy an EGL surface.
pub fn destroy_surface(display: EGLDisplay, surface: EGLSurface) -> EglCallResult<()> {
    if unsafe { ffi::eglDestroySurface(display, surface) } != EGL_TRUE {
        return Err(EglCallError::DestroySurface);
    }
    Ok(())
}

/// `[EGL 1.0]` Return information about an EGL frame buffer configuration.
pub fn get_config_attrib(display: EGLDisplay,
                         config: EGLConfig,
                         attribute: EGLint,
                         value: &mut EGLint)
                         -> EglCallResult<()> {
    if unsafe { ffi::eglGetConfigAttrib(display, config, attribute, value) } != EGL_TRUE {
        return Err(EglCallError::GetConfigAttrib);
    }
    Ok(())
}

/// `[EGL 1.0]` Return the total number of all available display configs.
///
/// On failure returns `None`.
pub fn num_configs(display: EGLDisplay) -> EglCallResult<i32> {
    let mut count: int32_t = 0;
    if unsafe { ffi::eglGetConfigs(display, ptr::null_mut(), 0, &mut count) } != EGL_TRUE {
        return Err(EglCallError::GetConfigs);
    }
    Ok(count as i32)
}

/// `[EGL 1.0]` Return a list of all EGL frame buffer configurations for a display.
///
/// Returns the number of configs written, `None` on failure.
pub fn get_configs(display: EGLDisplay, configs: &mut [EGLConfig]) -> EglCallResult<i32> {
    let mut count: int32_t = 0;
    if unsafe {
        ffi::eglGetConfigs(display,
                           mem::transmute(configs.as_mut_ptr()),
                           configs.len() as int32_t,
                           &mut count)
    } != EGL_TRUE {
        return Err(EglCallError::GetConfigs);
    }
    Ok(count as i32)
}

/// `[EGL 1.4]` Return the current EGL rendering context.
pub fn get_current_context() -> EglCallResult<EGLContext> {
    unsafe {
        let context = ffi::eglGetCurrentContext();

        if !context.is_null() {
            Ok(context)
        } else {
            Err(EglCallError::GetCurrentContext)
        }
    }
}

/// `[EGL 1.0]` Return the display for the current EGL rendering context.
pub fn get_current_display() -> EglCallResult<EGLDisplay> {
    unsafe {
        let display = ffi::eglGetCurrentDisplay();

        if !display.is_null() {
            Ok(display)
        } else {
            Err(EglCallError::GetCurrentDisplay)
        }
    }
}

/// `[EGL 1.0]` Return the read or draw surface for the current EGL rendering context.
pub fn get_current_surface(readdraw: EGLint) -> EglCallResult<EGLSurface> {
    unsafe {
        let surface = ffi::eglGetCurrentSurface(readdraw);

        if !surface.is_null() {
            Ok(surface)
        } else {
            Err(EglCallError::GetCurrentSurface)
        }
    }
}

/// `[EGL 1.0]` Return an EGL display connection.
pub fn get_display(display_id: EGLNativeDisplayType) -> EglCallResult<EGLDisplay> {
    unsafe {
        let display = ffi::eglGetDisplay(display_id);

        if !display.is_null() {
            Ok(display)
        } else {
            Err(EglCallError::GetDisplay)
        }
    }
}

/// `[EGL 1.0]` Return error information.
pub fn get_error() -> EGLint {
    unsafe { ffi::eglGetError() }
}

/// `[EGL 1.0]` Return a GL or an EGL extension function.
pub fn get_proc_address(procname: &str) -> extern "C" fn() {
    unsafe {
        let string = CString::new(procname).unwrap();

        ffi::eglGetProcAddress(string.as_ptr())
    }
}

/// `[EGL 1.0]` Initialize an EGL display connection.
pub fn initialize(display: EGLDisplay) -> EglCallResult<()> {
    if unsafe { ffi::eglInitialize(display, ptr::null_mut(), ptr::null_mut()) } != EGL_TRUE {
        return Err(EglCallError::Initialize);
    }
    Ok(())
}

/// `[EGL 1.0]` Initialize an EGL display connection and get EGL version.
pub fn initialize_and_get_version(display: EGLDisplay,
                                  major: &mut EGLint,
                                  minor: &mut EGLint)
                                  -> EglCallResult<()> {
    if unsafe { ffi::eglInitialize(display, major, minor) } != EGL_TRUE {
        return Err(EglCallError::Initialize);
    }
    Ok(())
}

/// `[EGL 1.0]` Attach an EGL rendering context to EGL surfaces.
pub fn make_current(display: EGLDisplay,
                    draw: EGLSurface,
                    read: EGLSurface,
                    ctx: EGLContext)
                    -> EglCallResult<()> {
    if unsafe { ffi::eglMakeCurrent(display, draw, read, ctx) } != EGL_TRUE {
        return Err(EglCallError::MakeCurrent);
    }
    Ok(())
}

/// `[EGL 1.2]` Query the current rendering API.
pub fn query_api() -> EGLenum {
    unsafe { ffi::eglQueryAPI() }
}

/// `[EGL 1.0]` Return EGL rendering context information.
pub fn query_context(display: EGLDisplay,
                     ctx: EGLContext,
                     attribute: EGLint,
                     value: &mut EGLint)
                     -> EglCallResult<()> {
    if unsafe { ffi::eglQueryContext(display, ctx, attribute, value) } != EGL_TRUE {
        return Err(EglCallError::QueryContext);
    }
    Ok(())
}

/// `[EGL 1.0]` Return a string describing an EGL display connection.
pub fn query_string(display: EGLDisplay, name: EGLint) -> EglCallResult<&'static CStr> {
    unsafe {
        let c_str = ffi::eglQueryString(display, name);

        if !c_str.is_null() {
            Ok(CStr::from_ptr(c_str))
        } else {
            Err(EglCallError::QueryString)
        }
    }
}

/// `[EGL 1.0]` Return EGL surface information.
pub fn query_surface(display: EGLDisplay,
                     surface: EGLSurface,
                     attribute: EGLint,
                     value: &mut EGLint)
                     -> EglCallResult<()> {
    if unsafe { ffi::eglQuerySurface(display, surface, attribute, value) } != EGL_TRUE {
        return Err(EglCallError::QuerySurface);
    }
    Ok(())
}

/// `[EGL 1.1]` Releases a color buffer that is being used as a texture.
pub fn release_tex_image(display: EGLDisplay,
                         surface: EGLSurface,
                         buffer: EGLint)
                         -> EglCallResult<()> {
    if unsafe { ffi::eglReleaseTexImage(display, surface, buffer) } != EGL_TRUE {
        return Err(EglCallError::ReleaseTexImage);
    }
    Ok(())
}

/// `[EGL 1.2]` Release EGL per-thread state.
pub fn release_thread() -> EglCallResult<()> {
    if unsafe { ffi::eglReleaseThread() } != EGL_TRUE {
        return Err(EglCallError::ReleaseThread);
    }
    Ok(())
}

/// `[EGL 1.1]` Set an EGL surface attribute.
pub fn surface_attrib(display: EGLDisplay,
                      surface: EGLSurface,
                      attribute: EGLint,
                      value: EGLint)
                      -> EglCallResult<()> {
    if unsafe { ffi::eglSurfaceAttrib(display, surface, attribute, value) } != EGL_TRUE {
        return Err(EglCallError::SurfaceAttrib);
    }
    Ok(())
}

/// `[EGL 1.0]` Post EGL surface color buffer to a native window.
pub fn swap_buffers(display: EGLDisplay, surface: EGLSurface) -> EglCallResult<()> {
    if unsafe { ffi::eglSwapBuffers(display, surface) } != EGL_TRUE {
        return Err(EglCallError::SwapBuffers);
    }
    Ok(())
}

/// `[EGL 1.1]` Specifies the minimum number of video frame periods per buffer swap for the window
/// associated with the current context.
pub fn swap_interval(display: EGLDisplay, interval: EGLint) -> EglCallResult<()> {
    if unsafe { ffi::eglSwapInterval(display, interval) } != EGL_TRUE {
        return Err(EglCallError::SwapInterval);
    }
    Ok(())
}

/// `[EGL 1.0]` Terminate an EGL display connection.
pub fn terminate(display: EGLDisplay) -> EglCallResult<()> {
    if unsafe { ffi::eglTerminate(display) } != EGL_TRUE {
        return Err(EglCallError::Terminate);
    }
    Ok(())
}

/// `[EGL 1.2]` Complete client API execution prior to subsequent native rendering calls.
pub fn wait_client() -> EglCallResult<()> {
    if unsafe { ffi::eglWaitClient() } != EGL_TRUE {
        return Err(EglCallError::WaitClient);
    }
    Ok(())
}

/// `[EGL 1.0]` Complete GL execution prior to subsequent native rendering calls.
pub fn wait_gl() -> EglCallResult<()> {
    if unsafe { ffi::eglWaitGL() } != EGL_TRUE {
        return Err(EglCallError::WaitGL);
    }
    Ok(())
}

/// `[EGL 1.0]` Complete native execution prior to subsequent GL rendering calls.
pub fn wait_native(engine: EGLint) -> EglCallResult<()> {
    if unsafe { ffi::eglWaitNative(engine) } != EGL_TRUE {
        return Err(EglCallError::WaitNative);
    }
    Ok(())
}
