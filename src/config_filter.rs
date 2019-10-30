// Copyright 2016 The EGLI Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::ptr;
use egl::{self, EGLDisplay, EGLint};
use error::Result;
use {FrameBufferConfigRef, ColorBufferType, ConfigCaveat, RenderableType, SurfaceType, TransparentType};

/// `[EGL 1.0]` Configuration filter builder.
pub struct ConfigFilterRef {
    handle: EGLDisplay,
    alpha_mask_size: Option<[EGLint; 2]>,
    alpha_size: Option<[EGLint; 2]>,
    bind_to_texture_rgb: Option<[EGLint; 2]>,
    bind_to_texture_rgba: Option<[EGLint; 2]>,
    blue_size: Option<[EGLint; 2]>,
    buffer_size: Option<[EGLint; 2]>,
    color_buffer_type: Option<[EGLint; 2]>,
    config_caveat: Option<[EGLint; 2]>,
    config_id: Option<[EGLint; 2]>,
    conformant: Option<[EGLint; 2]>,
    depth_size: Option<[EGLint; 2]>,
    green_size: Option<[EGLint; 2]>,
    level: Option<[EGLint; 2]>,
    luminance_size: Option<[EGLint; 2]>,
    match_native_pixmap: Option<[EGLint; 2]>,
    native_renderable: Option<[EGLint; 2]>,
    max_swap_interval: Option<[EGLint; 2]>,
    min_swap_interval: Option<[EGLint; 2]>,
    red_size: Option<[EGLint; 2]>,
    sample_buffers: Option<[EGLint; 2]>,
    samples: Option<[EGLint; 2]>,
    stencil_size: Option<[EGLint; 2]>,
    renderable_type: Option<[EGLint; 2]>,
    surface_type: Option<[EGLint; 2]>,
    transparent_type: Option<[EGLint; 2]>,
    transparent_red_value: Option<[EGLint; 2]>,
    transparent_green_value: Option<[EGLint; 2]>,
    transparent_blue_value: Option<[EGLint; 2]>,
}

impl ConfigFilterRef {
    pub fn from_native(handle: EGLDisplay) -> ConfigFilterRef {
        ConfigFilterRef {
            handle: handle,
            alpha_mask_size: None,
            alpha_size: None,
            bind_to_texture_rgb: None,
            bind_to_texture_rgba: None,
            blue_size: None,
            buffer_size: None,
            color_buffer_type: None,
            config_caveat: None,
            config_id: None,
            conformant: None,
            depth_size: None,
            green_size: None,
            level: None,
            luminance_size: None,
            match_native_pixmap: None,
            native_renderable: None,
            max_swap_interval: None,
            min_swap_interval: None,
            red_size: None,
            sample_buffers: None,
            samples: None,
            stencil_size: None,
            renderable_type: None,
            surface_type: None,
            transparent_type: None,
            transparent_red_value: None,
            transparent_green_value: None,
            transparent_blue_value: None,
        }
    }

    /// Must be followed by a nonnegative integer that indicates the desired alpha
    /// mask buffer size, in bits.
    /// The smallest alpha mask buffers of at least the specified size are preferred.
    /// The default value is zero.
    ///
    /// The alpha mask buffer is used only by OpenGL and OpenGL ES client APIs.
    pub fn with_alpha_mask_size(mut self, min_size: u32) -> Self {
        self.alpha_mask_size = Some([egl::EGL_ALPHA_MASK_SIZE, min_size as EGLint]);
        self
    }

    /// Must be followed by a nonnegative integer that indicates the desired size of the
    /// alpha component of the color buffer, in bits.
    /// If this value is zero, color buffers with the smallest alpha component size are
    /// preferred.
    /// Otherwise, color buffers with the largest alpha component of at least the specified
    /// size are preferred.
    /// The default value is zero.
    pub fn with_alpha_size(mut self, min_size: u32) -> Self {
        self.alpha_size = Some([egl::EGL_ALPHA_SIZE, min_size as EGLint]);
        self
    }

    /// Must be followed by `None`, `Some(true)`, or `Some(false)`.
    /// If `Some(true)` is specified, then only frame buffer configurations that
    /// support binding of color buffers to an OpenGL ES RGB texture will be considered.
    /// Currently only frame buffer configurations that support pbuffers allow this.
    /// The default value is `None`.
    pub fn with_bind_to_texture_rgb(mut self, value: Option<bool>) -> Self {
        self.bind_to_texture_rgb = Some([egl::EGL_BIND_TO_TEXTURE_RGB,
                                         match value {
                                             None => egl::EGL_DONT_CARE,
                                             Some(true) => egl::EGL_TRUE as EGLint,
                                             Some(false) => egl::EGL_FALSE as EGLint,
                                         }]);
        self
    }

    /// Must be followed by one of `None`, `Some(true)`, or `Some(false)`.
    /// If `Some(true)` is specified, then only frame buffer configurations that
    /// support binding of color buffers to an OpenGL ES RGBA texture will be
    /// considered.
    /// Currently only frame buffer configurations that support pbuffers allow this.
    /// The default value is `None`.
    pub fn with_bind_to_texture_rgba(mut self, value: Option<bool>) -> Self {
        self.bind_to_texture_rgba = Some([egl::EGL_BIND_TO_TEXTURE_RGBA,
                                          match value {
                                              None => egl::EGL_DONT_CARE,
                                              Some(true) => egl::EGL_TRUE as EGLint,
                                              Some(false) => egl::EGL_FALSE as EGLint,
                                          }]);
        self
    }

    /// Must be followed by a nonnegative integer that indicates the desired size
    /// of the blue component of the color buffer, in bits.
    /// If this value is zero, color buffers with the smallest blue component
    /// size are preferred. Otherwise, color buffers with the largest blue component
    /// of at least the specified size are preferred.
    /// The default value is zero.
    pub fn with_blue_size(mut self, min_size: u32) -> Self {
        self.blue_size = Some([egl::EGL_BLUE_SIZE, min_size as EGLint]);
        self
    }

    /// Must be followed by a nonnegative integer that indicates the desired color
    /// buffer size, in bits. The smallest color buffers of at least the specified
    /// size are preferred.
    /// The default value is zero.
    ///
    /// The color buffer size is the sum of EGL_RED_SIZE, EGL_GREEN_SIZE, EGL_BLUE_SIZE,
    /// and EGL_ALPHA_SIZE, and does not include any padding bits which may be present
    /// in the pixel format.
    /// It is usually preferable to specify desired sizes for these color components
    /// individually.
    pub fn with_buffer_size(mut self, min_size: u32) -> Self {
        self.buffer_size = Some([egl::EGL_BUFFER_SIZE, min_size as EGLint]);
        self
    }

    /// Must be followed by one of EGL_RGB_BUFFER or EGL_LUMINANCE_BUFFER.
    ///
    /// EGL_RGB_BUFFER indicates an RGB color buffer; in this case, attributes
    /// EGL_RED_SIZE, EGL_GREEN_SIZE and EGL_BLUE_SIZE must be non-zero,
    /// and EGL_LUMINANCE_SIZE must be zero.
    ///
    /// EGL_LUMINANCE_BUFFER indicates a luminance color buffer.
    /// In this case EGL_RED_SIZE, EGL_GREEN_SIZE, EGL_BLUE_SIZE must be zero,
    /// and EGL_LUMINANCE_SIZE must be non-zero.
    ///
    /// For both RGB and luminance color buffers, EGL_ALPHA_SIZE may be zero or non-zero.
    pub fn with_color_buffer_type(mut self, value: ColorBufferType) -> Self {
        self.color_buffer_type = Some([egl::EGL_COLOR_BUFFER_TYPE,
                                       match value {
                                           ColorBufferType::Rgb => egl::EGL_RGB_BUFFER,
                                           ColorBufferType::Luminance => egl::EGL_LUMINANCE_BUFFER,
                                       }]);
        self
    }

    /// Must be followed by `None`, `ConfigCaveat::None`, `ConfigCaveat::Slow`, or
    /// `ConfigCaveat::NonConformant`.
    ///
    /// If `None` is specified, then configs are not matched for this attribute.
    /// The default value is `None`.
    ///
    /// If `ConfigCaveat::None` is specified, then configs are matched for this attribute,
    /// but only configs with no caveats (neither `Slow` or `NonConformant`)
    /// will be considered.
    ///
    /// If `ConfigCaveat::Slow` is specified, then only slow configs configurations
    /// will be considered. The meaning of "slow" is implementation-dependent,
    /// but typically indicates a non-hardware-accelerated (software) implementation.
    ///
    /// If `ConfigCaveat::NonConformant` is specified, then only configs supporting
    /// non-conformant OpenGL ES contexts will be considered.
    ///
    /// If the EGL version is 1.3 or later, caveat `ConfigCaveat::NonConformant` is
    /// obsolete, since the same information can be specified via the EGL_CONFORMANT
    /// attribute on a per-client-API basis, not just for OpenGL ES.
    pub fn with_config_caveat(mut self, value: Option<ConfigCaveat>) -> Self {
        self.config_caveat = Some([egl::EGL_CONFIG_CAVEAT,
                                   match value {
                                       None => egl::EGL_DONT_CARE,
                                       Some(ConfigCaveat::None) => egl::EGL_NONE,
                                       Some(ConfigCaveat::Slow) => egl::EGL_SLOW_CONFIG,
                                       Some(ConfigCaveat::NonConformant) => {
                                           egl::EGL_NON_CONFORMANT_CONFIG
                                       }
                                   }]);
        self
    }

    /// Must be followed by a valid integer ID that indicates the desired EGL frame buffer
    /// configuration. When a EGL_CONFIG_ID is specified, all other attributes are ignored.
    /// The default value is `None`.
    ///
    /// The meaning of config IDs is implementation-dependent.
    /// They are used only to uniquely identify different frame buffer configurations.
    pub fn with_config_id(mut self, value: Option<i32>) -> Self {
        self.config_id = Some([egl::EGL_CONFIG_ID,
                               match value {
                                   Some(v) => v as EGLint,
                                   None => egl::EGL_DONT_CARE,
                               }]);
        self
    }

    /// Must be followed by a bitmask indicating which types of client API contexts
    /// created with respect to the frame buffer configuration config must pass the
    /// required conformance tests for that API. Mask bits include:
    ///
    /// ## RenderableType::OPENGL
    ///
    /// Config supports creating OpenGL contexts.
    ///
    /// ## RenderableType::OPENGL_ES
    ///
    /// Config supports creating OpenGL ES 1.0 and/or 1.1 contexts.
    ///
    /// ## RenderableType::OPENGL_ES2
    ///
    /// Config supports creating OpenGL ES 2.0 contexts.
    ///
    /// ## RenderableType::OPENVG
    ///
    /// Config supports creating OpenVG contexts.
    ///
    /// For example, if the bitmask is set to OPENGL_ES, only frame buffer
    /// configurations that support creating conformant OpenGL ES contexts will match.
    /// The default value is zero.
    ///
    /// Most EGLConfigs should be conformant for all supported client APIs, and it is rarely
    /// desirable to select a nonconformant config.
    /// Conformance requirements limit the number of non-conformant configs that an
    /// implementation can define.
    pub fn with_conformant(mut self, value: RenderableType) -> Self {
        self.conformant = Some([egl::EGL_CONFORMANT, value.bits() as EGLint]);
        self
    }

    /// Must be followed by a nonnegative integer that indicates the desired depth buffer
    /// size, in bits. The smallest depth buffers of at least the specified size is preferred.
    /// If the desired size is zero, frame buffer configurations with no depth buffer are
    /// preferred. The default value is zero.
    ///
    /// The depth buffer is used only by OpenGL and OpenGL ES client APIs.
    pub fn with_depth_size(mut self, min_size: u32) -> Self {
        self.depth_size = Some([egl::EGL_DEPTH_SIZE, min_size as EGLint]);
        self
    }

    /// Must be followed by a nonnegative integer that indicates the desired size of the green
    /// component of the color buffer, in bits.
    /// If this value is zero, color buffers with the smallest green component size are preferred.
    /// Otherwise, color buffers with the largest green component of at least the specified
    /// size are preferred. The default value is zero.
    pub fn with_green_size(mut self, min_size: u32) -> Self {
        self.green_size = Some([egl::EGL_GREEN_SIZE, min_size as EGLint]);
        self
    }

    /// Must be followed by an integer buffer level specification.
    /// This specification is honored exactly.
    /// Buffer level zero corresponds to the default frame buffer of the display.
    /// Buffer level one is the first overlay frame buffer, level two the second overlay frame
    /// buffer, and so on. Negative buffer levels correspond to underlay frame buffers.
    /// The default value is zero.
    ///
    /// Most imlementations do not support overlay or underlay planes
    /// (buffer levels other than zero).
    pub fn with_level(mut self, level: i32) -> Self {
        self.level = Some([egl::EGL_LEVEL, level as EGLint]);
        self
    }

    /// Must be followed by a nonnegative integer that indicates the desired
    /// size of the luminance component of the color buffer, in bits.
    /// If this value is zero, color buffers with the smallest luminance
    /// component size are preferred. Otherwise, color buffers with the largest
    /// luminance component of at least the specified size are preferred.
    /// The default value is zero.
    pub fn with_luminance_size(mut self, level: u32) -> Self {
        self.luminance_size = Some([egl::EGL_LUMINANCE_SIZE, level as EGLint]);
        self
    }

    /// Must be followed by the handle of a valid native pixmap, cast to `Some(i32)`, or `None`.
    /// If the value is not `None`, only configs which support creating pixmap surfaces with
    /// this pixmap using eglCreatePixmapSurface will match this attribute.
    /// If the value is `None`, then configs are not matched for this attribute.
    /// The default value is `None`.
    ///
    /// EGL_MATCH_NATIVE_PIXMAP was introduced due to the difficulty of determining an EGLConfig
    /// compatibile with a native pixmap using only color component sizes.
    pub fn with_match_native_pixmap(mut self, handle: Option<i32>) -> Self {
        self.match_native_pixmap = Some([egl::EGL_MATCH_NATIVE_PIXMAP,
                                         match handle {
                                             Some(v) => v as EGLint,
                                             None => egl::EGL_NONE,
                                         }]);
        self
    }

    /// Must be followed by EGL_DONT_CARE, EGL_TRUE, or EGL_FALSE. If EGL_TRUE is specified,
    /// then only frame buffer configurations that allow native rendering into the surface
    /// will be considered. The default value is EGL_DONT_CARE.
    pub fn with_native_renderable(mut self, value: Option<bool>) -> Self {
        self.native_renderable = Some([egl::EGL_NATIVE_RENDERABLE,
                                       match value {
                                           Some(true) => egl::EGL_TRUE as EGLint,
                                           Some(false) => egl::EGL_FALSE as EGLint,
                                           None => egl::EGL_DONT_CARE,
                                       }]);
        self
    }

    /// Must be followed by a integer that indicates the maximum value that can be passed to
    /// `eglSwapInterval`. The default value is `None`.
    pub fn with_max_swap_interval(mut self, value: Option<i32>) -> Self {
        self.max_swap_interval = Some([egl::EGL_MAX_SWAP_INTERVAL,
                                       match value {
                                           Some(value) => value as EGLint,
                                           None => egl::EGL_DONT_CARE,
                                       }]);
        self
    }

    /// Must be followed by a integer that indicates the minimum value that can be passed to
    /// `eglSwapInterval`. The default value is `None`.
    pub fn with_min_swap_interval(mut self, value: Option<i32>) -> Self {
        self.min_swap_interval = Some([egl::EGL_MIN_SWAP_INTERVAL,
                                       match value {
                                           Some(value) => value as EGLint,
                                           None => egl::EGL_DONT_CARE,
                                       }]);
        self
    }

    /// Must be followed by a nonnegative integer that indicates the desired size of the red
    /// component of the color buffer, in bits. If this value is zero, color buffers with the
    /// smallest red component size are preferred. Otherwise, color buffers with the largest
    /// red component of at least the specified size are preferred.
    /// The default value is zero.
    pub fn with_red_size(mut self, min_size: u32) -> Self {
        self.red_size = Some([egl::EGL_RED_SIZE, min_size as EGLint]);
        self
    }

    /// Must be followed by the minimum acceptable number of multisample buffers.
    /// Configurations with the smallest number of multisample buffers that meet or exceed
    /// this minimum number are preferred. Currently operation with more than one multisample
    /// buffer is undefined, so only values of zero or one will produce a match.
    /// The default value is zero.
    pub fn with_sample_buffers(mut self, value: i32) -> Self {
        self.sample_buffers = Some([egl::EGL_SAMPLE_BUFFERS, value as EGLint]);
        self
    }

    /// Must be followed by the minimum number of samples required in multisample buffers.
    /// Configurations with the smallest number of samples that meet or exceed the specified
    /// minimum number are preferred. Note that it is possible for color samples in the
    /// multisample buffer to have fewer bits than colors in the main color buffers.
    /// However, multisampled colors maintain at least as much color resolution in aggregate
    /// as the main color buffers.
    pub fn with_samples(mut self, value: i32) -> Self {
        self.samples = Some([egl::EGL_SAMPLES, value as EGLint]);
        self
    }

    /// Must be followed by a nonnegative integer that indicates the desired stencil buffer
    /// size, in bits. The smallest stencil buffers of at least the specified size are
    /// preferred. If the desired size is zero, frame buffer configurations with no
    /// stencil buffer are preferred. The default value is zero.
    ///
    /// The stencil buffer is used only by OpenGL and OpenGL ES client APIs.
    pub fn with_stencil_size(mut self, value: u32) -> Self {
        self.stencil_size = Some([egl::EGL_STENCIL_SIZE, value as EGLint]);
        self
    }

    /// Must be followed by a bitmask indicating which types of client API contexts the
    /// frame buffer configuration must support creating with eglCreateContext).
    /// Mask bits are the same as for attribute EGL_CONFORMANT.
    /// The default value is EGL_OPENGL_ES_BIT.
    pub fn with_renderable_type(mut self, value: RenderableType) -> Self {
        self.renderable_type = Some([egl::EGL_RENDERABLE_TYPE, value.bits() as EGLint]);
        self
    }

    /// Must be followed by a bitmask indicating which EGL surface types and capabilities
    /// the frame buffer configuration must support. Mask bits include:
    ///
    /// ## SurfaceType::MULTISAMPLE_RESOLVE_BOX
    ///
    /// Config allows specifying box filtered multisample resolve behavior with
    /// `eglSurfaceAttrib`.
    ///
    /// ## SurfaceType::PBUFFER
    ///
    /// Config supports creating pixel buffer surfaces.
    ///
    /// ## SurfaceType::PIXMAP
    ///
    /// Config supports creating pixmap surfaces.
    ///
    /// ## SurfaceType::SWAP_BEHAVIOR_PRESERVED
    ///
    /// Config allows setting swap behavior for color buffers with eglSurfaceAttrib.
    ///
    /// ## SurfaceType::VG_ALPHA_FORMAT_PRE
    ///
    /// Config allows specifying OpenVG rendering with premultiplied alpha values at surface
    /// creation time (see `eglCreatePbufferSurface`, `eglCreatePixmapSurface`, and
    /// `eglCreateWindowSurface`).
    ///
    /// ## SurfaceType::VG_COLORSPACE_LINEAR
    ///
    /// Config allows specifying OpenVG rendering in a linear colorspace at surface creation
    /// time (see `eglCreatePbufferSurface`, `eglCreatePixmapSurface`, and
    /// `eglCreateWindowSurface`).
    ///
    /// ## SurfaceType::WINDOW
    ///
    /// Config supports creating window surfaces.
    ///
    /// For example, if the bitmask is set to `SurfaceType::WINDOW` | `SurfaceType::PIXMAP`, only
    /// frame buffer configurations that support both windows and pixmaps will be considered.
    /// The default value is `SurfaceType::WINDOW`.
    pub fn with_surface_type(mut self, value: SurfaceType) -> Self {
        self.surface_type = Some([egl::EGL_SURFACE_TYPE, value.bits() as EGLint]);
        self
    }

    /// Must be followed by one of `TransparentType::None` or `TransparentType::TransparentRgb`.
    /// If `TransparentType::None` is specified, then only opaque frame buffer configurations
    /// will be considered. If `TransparentType::TransparentRgb` is specified, then only
    /// transparent frame buffer configurations will be considered.
    /// The default value is `TransparentType::None`.
    ///
    /// Most implementations support only opaque frame buffer configurations.
    pub fn with_transparent_type(mut self, value: TransparentType) -> Self {
        self.transparent_type = Some([egl::EGL_TRANSPARENT_TYPE,
                                      match value {
                                          TransparentType::None => egl::EGL_NONE,
                                          TransparentType::TransparentRgb => {
                                              egl::EGL_TRANSPARENT_RGB
                                          }
                                      }]);
        self
    }

    /// Must be followed by an integer value indicating the transparent red value.
    /// The value must be between zero and the maximum color buffer value for red.
    /// Only frame buffer configurations that use the specified transparent red value
    /// will be considered. The default value is `None`.
    ///
    /// This attribute is ignored unless `EGL_TRANSPARENT_TYPE` is included in attrib_list
    /// and specified as `TransparentType::TransparentRgb`.
    pub fn with_transparent_red_value(mut self, value: Option<u32>) -> Self {
        self.transparent_red_value = Some([egl::EGL_TRANSPARENT_RED_VALUE,
                                           match value {
                                               None => egl::EGL_DONT_CARE,
                                               Some(value) => value as EGLint,
                                           }]);
        self
    }

    /// Must be followed by an integer value indicating the transparent green value.
    /// The value must be between zero and the maximum color buffer value for green.
    /// Only frame buffer configurations that use the specified transparent green value
    /// will be considered. The default value is `None`.
    ///
    /// This attribute is ignored unless `EGL_TRANSPARENT_TYPE` is included in attrib_list
    /// and specified as `TransparentType::TransparentRgb`.
    pub fn with_transparent_green_value(mut self, value: Option<u32>) -> Self {
        self.transparent_green_value = Some([egl::EGL_TRANSPARENT_GREEN_VALUE,
                                             match value {
                                                 None => egl::EGL_DONT_CARE,
                                                 Some(value) => value as EGLint,
                                             }]);
        self
    }

    /// Must be followed by an integer value indicating the transparent blue value.
    /// The value must be between zero and the maximum color buffer value for blue.
    /// Only frame buffer configurations that use the specified transparent blue value
    /// will be considered. The default value is `None`.
    ///
    /// This attribute is ignored unless `EGL_TRANSPARENT_TYPE` is included in attrib_list
    /// and specified as `TransparentType::TransparentRgb`.
    pub fn with_transparent_blue_value(mut self, value: Option<u32>) -> Self {
        self.transparent_blue_value = Some([egl::EGL_TRANSPARENT_BLUE_VALUE,
                                            match value {
                                                None => egl::EGL_DONT_CARE,
                                                Some(value) => value as EGLint,
                                            }]);
        self
    }

    /// Get filtered display configurations.
    ///
    /// Internally, this calls `eglChooseConfig` twice: to get total filtered config count,
    /// and to fill the allocated memory with config handles.
    ///
    /// These handles are then wrapped into a new `Vec<FrameBufferConfigRef>`.
    pub fn choose_configs(self) -> Result<Vec<FrameBufferConfigRef>> {
        let attrib_list: Vec<_> = [self.alpha_mask_size,
                                   self.alpha_size,
                                   self.bind_to_texture_rgb,
                                   self.bind_to_texture_rgba,
                                   self.blue_size,
                                   self.buffer_size,
                                   self.color_buffer_type,
                                   self.config_caveat,
                                   self.config_id,
                                   self.conformant,
                                   self.depth_size,
                                   self.green_size,
                                   self.level,
                                   self.luminance_size,
                                   self.match_native_pixmap,
                                   self.native_renderable,
                                   self.max_swap_interval,
                                   self.min_swap_interval,
                                   self.red_size,
                                   self.sample_buffers,
                                   self.samples,
                                   self.stencil_size,
                                   self.renderable_type,
                                   self.surface_type,
                                   self.transparent_type,
                                   self.transparent_red_value,
                                   self.transparent_green_value,
                                   self.transparent_blue_value]
                                      .iter()
                                      .flat_map(|option| option)
                                      .flat_map(|arr| arr)
                                      .chain(&[egl::EGL_NONE])
                                      .cloned()
                                      .collect();

        let count = try!(egl::num_filtered_configs(self.handle, &attrib_list)) as usize;

        let mut configs: Vec<egl::EGLConfig> = vec![ptr::null_mut(); count];
        let returned_count =
            try!(egl::get_filtered_configs(self.handle, &attrib_list, &mut configs)) as usize;

        Ok(configs[..returned_count]
               .iter()
               .map(|c| FrameBufferConfigRef::from_native(self.handle, *c))
               .collect())
    }
}
