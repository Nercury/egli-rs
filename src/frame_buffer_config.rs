// Copyright 2016 The EGLI Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use egl;
use std::fmt;
use error::Result;
use {ColorBufferType, ConfigCaveat, RenderableType, SurfaceType, TransparentType};

/// `[EGL 1.0]` Reference to frame buffer configuration.
///
/// This is not a RAII object, so nothing will be cleaned up when it is dropped.
///
/// Instead, the individual methods of this object may return errors if they are used
/// after the display is terminated.
#[derive(Copy, Clone)]
pub struct FrameBufferConfigRef {
    display_handle: egl::EGLDisplay,
    config_handle: egl::EGLConfig,
}

impl FrameBufferConfigRef {
    pub fn from_native(display_id: egl::EGLDisplay,
                       config_handle: egl::EGLConfig)
                       -> FrameBufferConfigRef {
        FrameBufferConfigRef {
            display_handle: display_id,
            config_handle: config_handle,
        }
    }

    /// Get native config handle.
    pub fn handle(&self) -> egl::EGLConfig {
        self.config_handle
    }

    // Some methods bellow might not have correct return type
    // simply because I had not enough time to check them all!

    /// Returns the number of bits of alpha stored in the color buffer.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_ALPHA_SIZE` attribute.
    pub fn alpha_size(&self) -> Result<u32> {
        self.get_attrib(egl::EGL_ALPHA_SIZE)
            .map(|v| v as u32)
    }

    /// Returns the number of bits in the alpha mask buffer.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_ALPHA_MASK_SIZE` attribute.
    pub fn alpha_mask_size(&self) -> Result<u32> {
        self.get_attrib(egl::EGL_ALPHA_MASK_SIZE)
            .map(|v| v as u32)
    }

    /// Returns whether color buffers can be bound to an RGB texture.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_BIND_TO_TEXTURE_RGB` attribute.
    pub fn bind_to_texture_rgb(&self) -> Result<bool> {
        self.get_attrib(egl::EGL_BIND_TO_TEXTURE_RGB)
            .map(|v| (v as egl::EGLBoolean) == egl::EGL_TRUE)
    }

    /// Returns whether buffers can be bound to an RGBA texture.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_BIND_TO_TEXTURE_RGBA` attribute.
    pub fn bind_to_texture_rgba(&self) -> Result<bool> {
        self.get_attrib(egl::EGL_BIND_TO_TEXTURE_RGBA)
            .map(|v| (v as egl::EGLBoolean) == egl::EGL_TRUE)
    }

    /// Returns the number of bits of blue stored in the color buffer.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_BLUE_SIZE` attribute.
    pub fn blue_size(&self) -> Result<u32> {
        self.get_attrib(egl::EGL_BLUE_SIZE)
            .map(|v| v as u32)
    }

    /// Returns the depth of the color buffer.
    /// It is the sum of EGL_RED_SIZE, EGL_GREEN_SIZE, EGL_BLUE_SIZE,
    /// and EGL_ALPHA_SIZE.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_BUFFER_SIZE` attribute.
    pub fn buffer_size(&self) -> Result<u32> {
        self.get_attrib(egl::EGL_BUFFER_SIZE)
            .map(|v| v as u32)
    }

    /// Returns the color buffer type.
    /// Possible types are EGL_RGB_BUFFER and EGL_LUMINANCE_BUFFER.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_COLOR_BUFFER_TYPE` attribute.
    pub fn color_buffer_type(&self) -> Result<ColorBufferType> {
        self.get_attrib(egl::EGL_COLOR_BUFFER_TYPE)
            .map(|value| unsafe { ColorBufferType::from_raw(value) })
    }

    /// Returns the caveats for the frame buffer configuration.
    /// Possible caveat values are EGL_NONE, EGL_SLOW_CONFIG, and EGL_NON_CONFORMANT.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_CONFIG_CAVEAT` attribute.
    pub fn config_caveat(&self) -> Result<ConfigCaveat> {
        self.get_attrib(egl::EGL_CONFIG_CAVEAT)
            .map(|value| unsafe { ConfigCaveat::from_raw(value) })
    }

    /// Returns the ID of the frame buffer configuration.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_CONFIG_ID` attribute.
    pub fn config_id(&self) -> Result<i32> {
        self.get_attrib(egl::EGL_CONFIG_ID)
    }

    /// Returns a bitmask indicating which client API contexts created with respect to
    /// this config are conformant.
    ///
    /// EGL_CONFORMANT is supported only if the EGL version is 1.3 or greater.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_CONFORMANT` attribute.
    pub fn conformant(&self) -> Result<RenderableType> {
        self.get_attrib(egl::EGL_CONFORMANT)
            .map(|v| RenderableType::from_bits_truncate(v))
    }

    /// Returns the number of bits in the depth buffer.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_DEPTH_SIZE` attribute.
    pub fn depth_size(&self) -> Result<u32> {
        self.get_attrib(egl::EGL_DEPTH_SIZE)
            .map(|v| v as u32)
    }

    /// Returns the number of bits of green stored in the color buffer.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_GREEN_SIZE` attribute.
    pub fn green_size(&self) -> Result<u32> {
        self.get_attrib(egl::EGL_GREEN_SIZE)
            .map(|v| v as u32)
    }

    /// Returns the frame buffer level.
    /// Level zero is the default frame buffer.
    /// Positive levels correspond to frame buffers that overlay the default buffer and negative
    /// levels correspond to frame buffers that underlay the default buffer.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_LEVEL` attribute.
    pub fn level(&self) -> Result<i32> {
        self.get_attrib(egl::EGL_LEVEL)
    }

    /// Returns the number of bits of luminance stored in the luminance buffer.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_LUMINANCE_SIZE` attribute.
    pub fn luminance_size(&self) -> Result<u32> {
        self.get_attrib(egl::EGL_LUMINANCE_SIZE)
            .map(|v| v as u32)
    }

    /// Returns the maximum width of a pixel buffer surface in pixels.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_MAX_PBUFFER_WIDTH` attribute.
    pub fn max_pbuffer_width(&self) -> Result<i32> {
        self.get_attrib(egl::EGL_MAX_PBUFFER_WIDTH)
    }

    /// Returns the maximum height of a pixel buffer surface in pixels.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_MAX_PBUFFER_HEIGHT` attribute.
    pub fn max_pbuffer_height(&self) -> Result<i32> {
        self.get_attrib(egl::EGL_MAX_PBUFFER_HEIGHT)
    }

    /// Returns the maximum size of a pixel buffer surface in pixels.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_MAX_PBUFFER_PIXELS` attribute.
    pub fn max_pbuffer_pixels(&self) -> Result<i32> {
        self.get_attrib(egl::EGL_MAX_PBUFFER_PIXELS)
    }

    /// Returns the maximum value that can be passed to eglSwapInterval.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_MAX_SWAP_INTERVAL` attribute.
    pub fn max_swap_interval(&self) -> Result<i32> {
        self.get_attrib(egl::EGL_MAX_SWAP_INTERVAL)
    }

    /// Returns the minimum value that can be passed to eglSwapInterval.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_MIN_SWAP_INTERVAL` attribute.
    pub fn min_swap_interval(&self) -> Result<i32> {
        self.get_attrib(egl::EGL_MIN_SWAP_INTERVAL)
    }

    /// Returns whether native rendering APIs can render into the surface.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_NATIVE_RENDERABLE` attribute.
    pub fn native_renderable(&self) -> Result<bool> {
        self.get_attrib(egl::EGL_NATIVE_RENDERABLE)
            .map(|v| (v as egl::EGLBoolean) == egl::EGL_TRUE)
    }

    /// Returns the ID of the associated native visual.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_NATIVE_VISUAL_ID` attribute.
    pub fn native_visual_id(&self) -> Result<i32> {
        self.get_attrib(egl::EGL_NATIVE_VISUAL_ID)
    }

    /// Returns the type of the associated native visual.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_NATIVE_VISUAL_TYPE` attribute.
    pub fn native_visual_type(&self) -> Result<i32> {
        self.get_attrib(egl::EGL_NATIVE_VISUAL_TYPE)
    }

    /// Returns the number of bits of red stored in the color buffer.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_RED_SIZE` attribute.
    pub fn red_size(&self) -> Result<u32> {
        self.get_attrib(egl::EGL_RED_SIZE)
            .map(|v| v as u32)
    }

    /// Returns a bitmask indicating the types of supported client API contexts.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_RENDERABLE_TYPE` attribute.
    pub fn renderable_type(&self) -> Result<RenderableType> {
        self.get_attrib(egl::EGL_RENDERABLE_TYPE)
            .map(|v| RenderableType::from_bits_truncate(v))
    }

    /// Returns the number of multisample buffers.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_SAMPLE_BUFFERS` attribute.
    pub fn sample_buffers(&self) -> Result<i32> {
        self.get_attrib(egl::EGL_SAMPLE_BUFFERS)
    }

    /// Returns the number of samples per pixel.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_SAMPLES` attribute.
    pub fn samples(&self) -> Result<i32> {
        self.get_attrib(egl::EGL_SAMPLES)
    }

    /// Returns the number of bits in the stencil buffer.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_STENCIL_SIZE` attribute.
    pub fn stencil_size(&self) -> Result<u32> {
        self.get_attrib(egl::EGL_STENCIL_SIZE)
            .map(|v| v as u32)
    }

    /// Returns a bitmask indicating the types of supported EGL surfaces.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_SURFACE_TYPE` attribute.
    pub fn surface_type(&self) -> Result<SurfaceType> {
        self.get_attrib(egl::EGL_SURFACE_TYPE)
            .map(|v| SurfaceType::from_bits_truncate(v))
    }

    /// Returns the type of supported transparency.
    /// Possible transparency values are: EGL_NONE, and EGL_TRANSPARENT_RGB.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_TRANSPARENT_TYPE` attribute.
    pub fn transparent_type(&self) -> Result<TransparentType> {
        self.get_attrib(egl::EGL_TRANSPARENT_TYPE)
            .map(|value| unsafe { TransparentType::from_raw(value) })
    }

    /// Returns the transparent red value.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_TRANSPARENT_RED_VALUE` attribute.
    pub fn transparent_red_value(&self) -> Result<u32> {
        self.get_attrib(egl::EGL_TRANSPARENT_RED_VALUE)
            .map(|value| value as u32)
    }

    /// Returns the transparent green value.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_TRANSPARENT_GREEN_VALUE` attribute.
    pub fn transparent_green_value(&self) -> Result<u32> {
        self.get_attrib(egl::EGL_TRANSPARENT_GREEN_VALUE)
            .map(|value| value as u32)
    }

    /// Returns the transparent blue value.
    ///
    /// Calls `eglGetConfigAttrib` with `EGL_TRANSPARENT_BLUE_VALUE` attribute.
    pub fn transparent_blue_value(&self) -> Result<u32> {
        self.get_attrib(egl::EGL_TRANSPARENT_GREEN_VALUE)
            .map(|value| value as u32)
    }

    fn get_attrib(&self, attribute: egl::EGLint) -> Result<egl::EGLint> {
        let mut value: egl::EGLint = 0;
        try!(egl::get_config_attrib(self.display_handle,
                                    self.config_handle,
                                    attribute,
                                    &mut value));
        Ok(value)
    }

    fn format_debug_struct(&self, f: &mut fmt::Formatter) -> Result<fmt::Result> {
        Ok(f.debug_struct("FrameBufferConfigRef")
            .field("config_id", &try!(self.config_id()))
            .field("red_size", &try!(self.red_size()))
            .field("green_size", &try!(self.green_size()))
            .field("blue_size", &try!(self.blue_size()))
            .field("alpha_size", &try!(self.alpha_size()))
            .field("buffer_size", &try!(self.buffer_size()))
            .field("alpha_mask_size", &try!(self.alpha_mask_size()))
            .field("depth_size", &try!(self.depth_size()))
            .field("stencil_size", &try!(self.stencil_size()))
            .field("bind_to_texture_rgb", &try!(self.bind_to_texture_rgb()))
            .field("bind_to_texture_rgba", &try!(self.bind_to_texture_rgba()))
            .field("color_buffer_type", &try!(self.color_buffer_type()))
            .field("config_caveat", &try!(self.config_caveat()))
            .field("conformant", &try!(self.conformant()))
            .field("level", &try!(self.level()))
            .field("luminance_size", &try!(self.luminance_size()))
            .field("max_pbuffer_width", &try!(self.max_pbuffer_width()))
            .field("max_pbuffer_height", &try!(self.max_pbuffer_height()))
            .field("max_pbuffer_pixels", &try!(self.max_pbuffer_pixels()))
            .field("max_swap_interval", &try!(self.max_swap_interval()))
            .field("min_swap_interval", &try!(self.min_swap_interval()))
            .field("native_renderable", &try!(self.native_renderable()))
            .field("native_visual_id", &try!(self.native_visual_id()))
            .field("native_visual_type", &try!(self.native_visual_type()))
            .field("renderable_type", &try!(self.renderable_type()))
            .field("sample_buffers", &try!(self.sample_buffers()))
            .field("samples", &try!(self.samples()))
            .field("surface_type", &try!(self.surface_type()))
            .field("transparent_type", &try!(self.transparent_type()))
            .field("transparent_red_value", &try!(self.transparent_red_value()))
            .field("transparent_green_value",
                   &try!(self.transparent_green_value()))
            .field("transparent_blue_value",
                   &try!(self.transparent_blue_value()))
            .finish())
    }
}

impl fmt::Debug for FrameBufferConfigRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.format_debug_struct(f) {
            Ok(result) => result,
            Err(e) => {
                f.debug_struct("FrameBufferConfigRef")
                 .field("error", &format!("{:?}", e))
                 .finish()
            }
        }
    }
}
