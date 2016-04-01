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

#![allow(non_camel_case_types)]

use libc::{c_float, int8_t, int16_t, int32_t, int64_t, uint8_t, uint16_t, uint32_t, uint64_t};

// -------------------------------------------------------------------------------------------------
// TYPES
// -------------------------------------------------------------------------------------------------

pub type khronos_float_t = c_float;
pub type khronos_int8_t = int8_t;
pub type khronos_uint8_t = uint8_t;
pub type khronos_int16_t = int16_t;
pub type khronos_uint16_t = uint16_t;
pub type khronos_int32_t = int32_t;
pub type khronos_uint32_t = uint32_t;
pub type khronos_int64_t = int64_t;
pub type khronos_uint64_t = uint64_t;

pub type khronos_intptr_t = int32_t;
pub type khronos_uintptr_t = uint32_t;
pub type khronos_ssize_t = int32_t;
pub type khronos_usize_t = uint32_t;

pub type khronos_stime_nanoseconds_t = khronos_int64_t;
pub type khronos_utime_nanoseconds_t = khronos_uint64_t;
