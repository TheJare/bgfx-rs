// Copyright (c) 2015-2016, Johan Sköld.
// License: http://opensource.org/licenses/ISC

//! Raw FFI bgfx bindings.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

use std::fmt;


pub type size_t   = ::libc::size_t;
pub type int32_t  = i32;
pub type uint8_t  = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;

include!("ffi_platform.rs");

pub const BGFX_PCI_ID_NONE:                 u16 = 0x0000;
pub const BGFX_PCI_ID_SOFTWARE_RASTERIZER:  u16 = 0x0001;
pub const BGFX_PCI_ID_AMD:                  u16 = 0x1002;
pub const BGFX_PCI_ID_INTEL:                u16 = 0x8086;
pub const BGFX_PCI_ID_NVIDIA:               u16 = 0x10de;

/// Bgfx Caps

pub const BGFX_CAPS_ALPHA_TO_COVERAGE:      u64 = 0x0000000000000001;
pub const BGFX_CAPS_BLEND_INDEPENDENT:      u64 = 0x0000000000000002;
pub const BGFX_CAPS_COMPUTE:                u64 = 0x0000000000000004;
pub const BGFX_CAPS_CONSERVATIVE_RASTER:    u64 = 0x0000000000000008;
pub const BGFX_CAPS_DRAW_INDIRECT:          u64 = 0x0000000000000010;
pub const BGFX_CAPS_FRAGMENT_DEPTH:         u64 = 0x0000000000000020;
pub const BGFX_CAPS_FRAGMENT_ORDERING:      u64 = 0x0000000000000040;
pub const BGFX_CAPS_GRAPHICS_DEBUGGER:      u64 = 0x0000000000000080;
pub const BGFX_CAPS_HIDPI:                  u64 = 0x0000000000000100;
pub const BGFX_CAPS_HMD:                    u64 = 0x0000000000000200;
pub const BGFX_CAPS_INDEX32:                u64 = 0x0000000000000400;
pub const BGFX_CAPS_INSTANCING:             u64 = 0x0000000000000800;
pub const BGFX_CAPS_OCCLUSION_QUERY:        u64 = 0x0000000000001000;
pub const BGFX_CAPS_RENDERER_MULTITHREADED: u64 = 0x0000000000002000;
pub const BGFX_CAPS_SWAP_CHAIN:             u64 = 0x0000000000004000;
pub const BGFX_CAPS_TEXTURE_2D_ARRAY:       u64 = 0x0000000000008000;
pub const BGFX_CAPS_TEXTURE_3D:             u64 = 0x0000000000010000;
pub const BGFX_CAPS_TEXTURE_BLIT:           u64 = 0x0000000000020000;
pub const BGFX_CAPS_TEXTURE_COMPARE_ALL:    u64 = 0x00000000000c0000;
pub const BGFX_CAPS_TEXTURE_COMPARE_LEQUAL: u64 = 0x0000000000080000;
pub const BGFX_CAPS_TEXTURE_CUBE_ARRAY:     u64 = 0x0000000000100000;
pub const BGFX_CAPS_TEXTURE_READ_BACK:      u64 = 0x0000000000200000;
pub const BGFX_CAPS_VERTEX_ATTRIB_HALF:     u64 = 0x0000000000400000;
pub const BGFX_CAPS_VERTEX_ATTRIB_UINT10:   u64 = 0x0000000000800000;

/// Texture Caps

pub const BGFX_CAPS_FORMAT_TEXTURE_NONE:             u16 = 0x0000;
pub const BGFX_CAPS_FORMAT_TEXTURE_2D:               u16 = 0x0001;
pub const BGFX_CAPS_FORMAT_TEXTURE_2D_SRGB:          u16 = 0x0002;
pub const BGFX_CAPS_FORMAT_TEXTURE_2D_EMULATED:      u16 = 0x0004;
pub const BGFX_CAPS_FORMAT_TEXTURE_3D:               u16 = 0x0008;
pub const BGFX_CAPS_FORMAT_TEXTURE_3D_SRGB:          u16 = 0x0010;
pub const BGFX_CAPS_FORMAT_TEXTURE_3D_EMULATED:      u16 = 0x0020;
pub const BGFX_CAPS_FORMAT_TEXTURE_CUBE:             u16 = 0x0040;
pub const BGFX_CAPS_FORMAT_TEXTURE_CUBE_SRGB:        u16 = 0x0080;
pub const BGFX_CAPS_FORMAT_TEXTURE_CUBE_EMULATED:    u16 = 0x0100;
pub const BGFX_CAPS_FORMAT_TEXTURE_VERTEX:           u16 = 0x0200;
pub const BGFX_CAPS_FORMAT_TEXTURE_IMAGE:            u16 = 0x0400;
pub const BGFX_CAPS_FORMAT_TEXTURE_FRAMEBUFFER:      u16 = 0x0800;
pub const BGFX_CAPS_FORMAT_TEXTURE_FRAMEBUFFER_MSAA: u16 = 0x1000;
pub const BGFX_CAPS_FORMAT_TEXTURE_MSAA:             u16 = 0x2000;
pub const BGFX_CAPS_FORMAT_TEXTURE_MIP_AUTOGEN:      u16 = 0x4000;

// Clear flags

pub const BGFX_CLEAR_NONE:                  u16 = 0x0000;
pub const BGFX_CLEAR_COLOR:                 u16 = 0x0001;
pub const BGFX_CLEAR_DEPTH:                 u16 = 0x0002;
pub const BGFX_CLEAR_STENCIL:               u16 = 0x0004;
pub const BGFX_CLEAR_DISCARD_COLOR_0:       u16 = 0x0008;
pub const BGFX_CLEAR_DISCARD_COLOR_1:       u16 = 0x0010;
pub const BGFX_CLEAR_DISCARD_COLOR_2:       u16 = 0x0020;
pub const BGFX_CLEAR_DISCARD_COLOR_3:       u16 = 0x0040;
pub const BGFX_CLEAR_DISCARD_COLOR_4:       u16 = 0x0080;
pub const BGFX_CLEAR_DISCARD_COLOR_5:       u16 = 0x0100;
pub const BGFX_CLEAR_DISCARD_COLOR_6:       u16 = 0x0200;
pub const BGFX_CLEAR_DISCARD_COLOR_7:       u16 = 0x0400;
pub const BGFX_CLEAR_DISCARD_DEPTH:         u16 = 0x0800;
pub const BGFX_CLEAR_DISCARD_STENCIL:       u16 = 0x1000;

pub const BGFX_CLEAR_DISCARD_COLOR_MASK:    u16 =
    (
        BGFX_CLEAR_DISCARD_COLOR_0 |
        BGFX_CLEAR_DISCARD_COLOR_1 |
        BGFX_CLEAR_DISCARD_COLOR_2 |
        BGFX_CLEAR_DISCARD_COLOR_3 |
        BGFX_CLEAR_DISCARD_COLOR_4 |
        BGFX_CLEAR_DISCARD_COLOR_5 |
        BGFX_CLEAR_DISCARD_COLOR_6 |
        BGFX_CLEAR_DISCARD_COLOR_7
    );

pub const BGFX_CLEAR_DISCARD_MASK:          u16 =
    (
        BGFX_CLEAR_DISCARD_COLOR_MASK |
        BGFX_CLEAR_DISCARD_DEPTH |
        BGFX_CLEAR_DISCARD_STENCIL
    );

// Debug flags

pub const BGFX_DEBUG_NONE:                  u32 = 0x00000000;
pub const BGFX_DEBUG_WIREFRAME:             u32 = 0x00000001;
pub const BGFX_DEBUG_IFH:                   u32 = 0x00000002;
pub const BGFX_DEBUG_STATS:                 u32 = 0x00000004;
pub const BGFX_DEBUG_TEXT:                  u32 = 0x00000008;

// Reset flags

pub const BGFX_RESET_NONE:                  u32 = 0x00000000;
pub const BGFX_RESET_FULLSCREEN:            u32 = 0x00000001;
pub const BGFX_RESET_FULLSCREEN_MASK:       u32 = 0x00000001;
pub const BGFX_RESET_MSAA_X2:               u32 = 0x00000010;
pub const BGFX_RESET_MSAA_X4:               u32 = 0x00000020;
pub const BGFX_RESET_MSAA_X8:               u32 = 0x00000030;
pub const BGFX_RESET_MSAA_X16:              u32 = 0x00000040;
pub const BGFX_RESET_MSAA_MASK:             u32 = 0x00000070;
pub const BGFX_RESET_VSYNC:                 u32 = 0x00000080;
pub const BGFX_RESET_MAXANISOTROPY:         u32 = 0x00000100;
pub const BGFX_RESET_CAPTURE:               u32 = 0x00000200;
pub const BGFX_RESET_HMD:                   u32 = 0x00000400;
pub const BGFX_RESET_HMD_DEBUG:             u32 = 0x00000800;
pub const BGFX_RESET_HMD_RECENTER:          u32 = 0x00001000;
pub const BGFX_RESET_FLUSH_AFTER_RENDER:    u32 = 0x00002000;
pub const BGFX_RESET_FLIP_AFTER_RENDER:     u32 = 0x00004000;
pub const BGFX_RESET_SRGB_BACKBUFFER:       u32 = 0x00008000;
pub const BGFX_RESET_HIDPI:                 u32 = 0x00010000;
pub const BGFX_RESET_DEPTH_CLAMP:           u32 = 0x00020000;
pub const BGFX_RESET_SUSPEND:               u32 = 0x00040000;

// Buffer flags

pub const BGFX_BUFFER_NONE:                 u16 = 0x0000;
pub const BGFX_BUFFER_COMPUTE_FORMAT_8X1:   u16 = 0x0001;
pub const BGFX_BUFFER_COMPUTE_FORMAT_8X2:   u16 = 0x0002;
pub const BGFX_BUFFER_COMPUTE_FORMAT_8X4:   u16 = 0x0003;
pub const BGFX_BUFFER_COMPUTE_FORMAT_16X1:  u16 = 0x0004;
pub const BGFX_BUFFER_COMPUTE_FORMAT_16X2:  u16 = 0x0005;
pub const BGFX_BUFFER_COMPUTE_FORMAT_16X4:  u16 = 0x0006;
pub const BGFX_BUFFER_COMPUTE_FORMAT_32X1:  u16 = 0x0007;
pub const BGFX_BUFFER_COMPUTE_FORMAT_32X2:  u16 = 0x0008;
pub const BGFX_BUFFER_COMPUTE_FORMAT_32X4:  u16 = 0x0009;
pub const BGFX_BUFFER_COMPUTE_FORMAT_MASK:  u16 = 0x000f;
pub const BGFX_BUFFER_COMPUTE_TYPE_UINT:    u16 = 0x0010;
pub const BGFX_BUFFER_COMPUTE_TYPE_INT:     u16 = 0x0020;
pub const BGFX_BUFFER_COMPUTE_TYPE_FLOAT:   u16 = 0x0030;
pub const BGFX_BUFFER_COMPUTE_TYPE_MASK:    u16 = 0x0030;
pub const BGFX_BUFFER_COMPUTE_READ:         u16 = 0x0100;
pub const BGFX_BUFFER_COMPUTE_WRITE:        u16 = 0x0200;
pub const BGFX_BUFFER_DRAW_INDIRECT:        u16 = 0x0400;
pub const BGFX_BUFFER_ALLOW_RESIZE:         u16 = 0x0800;
pub const BGFX_BUFFER_INDEX32:              u16 = 0x1000;

pub const BGFX_BUFFER_COMPUTE_READ_WRITE:   u16 =
    (
        BGFX_BUFFER_COMPUTE_READ |
        BGFX_BUFFER_COMPUTE_WRITE
    );

// Texture flags

pub const BGFX_TEXTURE_NONE:                u32 = 0x00000000;
pub const BGFX_TEXTURE_U_MIRROR:            u32 = 0x00000001;
pub const BGFX_TEXTURE_U_CLAMP:             u32 = 0x00000002;
pub const BGFX_TEXTURE_U_BORDER:            u32 = 0x00000003;
pub const BGFX_TEXTURE_U_MASK:              u32 = 0x00000003;
pub const BGFX_TEXTURE_V_MIRROR:            u32 = 0x00000004;
pub const BGFX_TEXTURE_V_CLAMP:             u32 = 0x00000008;
pub const BGFX_TEXTURE_V_BORDER:            u32 = 0x0000000c;
pub const BGFX_TEXTURE_V_MASK:              u32 = 0x0000000c;
pub const BGFX_TEXTURE_W_MIRROR:            u32 = 0x00000010;
pub const BGFX_TEXTURE_W_CLAMP:             u32 = 0x00000020;
pub const BGFX_TEXTURE_W_BORDER:            u32 = 0x00000030;
pub const BGFX_TEXTURE_W_MASK:              u32 = 0x00000030;
pub const BGFX_TEXTURE_MIN_POINT:           u32 = 0x00000040;
pub const BGFX_TEXTURE_MIN_ANISOTROPIC:     u32 = 0x00000080;
pub const BGFX_TEXTURE_MIN_MASK:            u32 = 0x000000c0;
pub const BGFX_TEXTURE_MAG_POINT:           u32 = 0x00000100;
pub const BGFX_TEXTURE_MAG_ANISOTROPIC:     u32 = 0x00000200;
pub const BGFX_TEXTURE_MAG_MASK:            u32 = 0x00000300;
pub const BGFX_TEXTURE_MIP_POINT:           u32 = 0x00000400;
pub const BGFX_TEXTURE_MIP_MASK:            u32 = 0x00000400;
pub const BGFX_TEXTURE_MSAA_SAMPLE:         u32 = 0x00000800;
pub const BGFX_TEXTURE_RT:                  u32 = 0x00001000;
pub const BGFX_TEXTURE_RT_MSAA_X2:          u32 = 0x00002000;
pub const BGFX_TEXTURE_RT_MSAA_X4:          u32 = 0x00003000;
pub const BGFX_TEXTURE_RT_MSAA_X8:          u32 = 0x00004000;
pub const BGFX_TEXTURE_RT_MSAA_X16:         u32 = 0x00005000;
pub const BGFX_TEXTURE_RT_MSAA_MASK:        u32 = 0x00007000;
pub const BGFX_TEXTURE_RT_WRITE_ONLY:       u32 = 0x00008000;
pub const BGFX_TEXTURE_RT_MASK:             u32 = 0x0000f000;
pub const BGFX_TEXTURE_COMPARE_LESS:        u32 = 0x00010000;
pub const BGFX_TEXTURE_COMPARE_LEQUAL:      u32 = 0x00020000;
pub const BGFX_TEXTURE_COMPARE_EQUAL:       u32 = 0x00030000;
pub const BGFX_TEXTURE_COMPARE_GEQUAL:      u32 = 0x00040000;
pub const BGFX_TEXTURE_COMPARE_GREATER:     u32 = 0x00050000;
pub const BGFX_TEXTURE_COMPARE_NOTEQUAL:    u32 = 0x00060000;
pub const BGFX_TEXTURE_COMPARE_NEVER:       u32 = 0x00070000;
pub const BGFX_TEXTURE_COMPARE_ALWAYS:      u32 = 0x00080000;
pub const BGFX_TEXTURE_COMPARE_MASK:        u32 = 0x000f0000;
pub const BGFX_TEXTURE_COMPUTE_WRITE:       u32 = 0x00100000;
pub const BGFX_TEXTURE_SRGB:                u32 = 0x00200000;
pub const BGFX_TEXTURE_BLIT_DST:            u32 = 0x00400000;
pub const BGFX_TEXTURE_READ_BACK:           u32 = 0x00800000;
pub const BGFX_TEXTURE_BORDER_COLOR_MASK:   u32 = 0x0f000000;
pub const BGFX_TEXTURE_RESERVED_MASK:       u32 = 0xf0000000;

#[macro_export]
macro_rules! BGFX_TEXTURE_BORDER_COLOR {
    ($aref:expr) => ((($aref as u32) << bgfx_sys::BGFX_TEXTURE_BORDER_COLOR_SHIFT) & bgfx_sys::BGFX_TEXTURE_BORDER_COLOR_MASK)
}

pub const BGFX_TEXTURE_SAMPLER_BITS_MASK: u32 =
    (
        BGFX_TEXTURE_U_MASK |
        BGFX_TEXTURE_V_MASK |
        BGFX_TEXTURE_W_MASK |
        BGFX_TEXTURE_MIN_MASK |
        BGFX_TEXTURE_MAG_MASK |
        BGFX_TEXTURE_MIP_MASK |
        BGFX_TEXTURE_COMPARE_MASK
    );

///
pub const BGFX_VIEW_NONE:   u8 = 0x00;
pub const BGFX_VIEW_STEREO: u8 = 0x01;

///
pub const BGFX_SUBMIT_EYE_LEFT:       u8 = 0x01;
pub const BGFX_SUBMIT_EYE_RIGHT:      u8 = 0x02;
pub const BGFX_SUBMIT_EYE_MASK:       u8 = 0x03;
pub const BGFX_SUBMIT_EYE_FIRST:      u8 = BGFX_SUBMIT_EYE_LEFT;
pub const BGFX_SUBMIT_RESERVED_MASK:  u8 = 0x80;

///
pub const BGFX_HMD_NONE:              u8 = 0x00;
pub const BGFX_HMD_DEVICE_RESOLUTION: u8 = 0x01;
pub const BGFX_HMD_RENDERING:         u8 = 0x02;

///
pub const BGFX_CUBE_MAP_POSITIVE_X: u8 = 0x00;
pub const BGFX_CUBE_MAP_NEGATIVE_X: u8 = 0x01;
pub const BGFX_CUBE_MAP_POSITIVE_Y: u8 = 0x02;
pub const BGFX_CUBE_MAP_NEGATIVE_Y: u8 = 0x03;
pub const BGFX_CUBE_MAP_POSITIVE_Z: u8 = 0x04;
pub const BGFX_CUBE_MAP_NEGATIVE_Z: u8 = 0x05;

// State flags

pub const BGFX_STATE_RGB_WRITE:             u64 = 0x0000000000000001_u64;
pub const BGFX_STATE_ALPHA_WRITE:           u64 = 0x0000000000000002_u64;
pub const BGFX_STATE_DEPTH_WRITE:           u64 = 0x0000000000000004_u64;
pub const BGFX_STATE_DEPTH_TEST_LESS:       u64 = 0x0000000000000010_u64;
pub const BGFX_STATE_DEPTH_TEST_LEQUAL:     u64 = 0x0000000000000020_u64;
pub const BGFX_STATE_DEPTH_TEST_EQUAL:      u64 = 0x0000000000000030_u64;
pub const BGFX_STATE_DEPTH_TEST_GEQUAL:     u64 = 0x0000000000000040_u64;
pub const BGFX_STATE_DEPTH_TEST_GREATER:    u64 = 0x0000000000000050_u64;
pub const BGFX_STATE_DEPTH_TEST_NOTEQUAL:   u64 = 0x0000000000000060_u64;
pub const BGFX_STATE_DEPTH_TEST_NEVER:      u64 = 0x0000000000000070_u64;
pub const BGFX_STATE_DEPTH_TEST_ALWAYS:     u64 = 0x0000000000000080_u64;
pub const BGFX_STATE_DEPTH_TEST_MASK:       u64 = 0x00000000000000f0_u64;
pub const BGFX_STATE_BLEND_ZERO:            u64 = 0x0000000000001000_u64;
pub const BGFX_STATE_BLEND_ONE:             u64 = 0x0000000000002000_u64;
pub const BGFX_STATE_BLEND_SRC_COLOR:       u64 = 0x0000000000003000_u64;
pub const BGFX_STATE_BLEND_INV_SRC_COLOR:   u64 = 0x0000000000004000_u64;
pub const BGFX_STATE_BLEND_SRC_ALPHA:       u64 = 0x0000000000005000_u64;
pub const BGFX_STATE_BLEND_INV_SRC_ALPHA:   u64 = 0x0000000000006000_u64;
pub const BGFX_STATE_BLEND_DST_ALPHA:       u64 = 0x0000000000007000_u64;
pub const BGFX_STATE_BLEND_INV_DST_ALPHA:   u64 = 0x0000000000008000_u64;
pub const BGFX_STATE_BLEND_DST_COLOR:       u64 = 0x0000000000009000_u64;
pub const BGFX_STATE_BLEND_INV_DST_COLOR:   u64 = 0x000000000000a000_u64;
pub const BGFX_STATE_BLEND_SRC_ALPHA_SAT:   u64 = 0x000000000000b000_u64;
pub const BGFX_STATE_BLEND_FACTOR:          u64 = 0x000000000000c000_u64;
pub const BGFX_STATE_BLEND_INV_FACTOR:      u64 = 0x000000000000d000_u64;
pub const BGFX_STATE_BLEND_MASK:            u64 = 0x000000000ffff000_u64;
pub const BGFX_STATE_BLEND_EQUATION_ADD:    u64 = 0x0000000000000000_u64;
pub const BGFX_STATE_BLEND_EQUATION_SUB:    u64 = 0x0000000010000000_u64;
pub const BGFX_STATE_BLEND_EQUATION_REVSUB: u64 = 0x0000000020000000_u64;
pub const BGFX_STATE_BLEND_EQUATION_MIN:    u64 = 0x0000000030000000_u64;
pub const BGFX_STATE_BLEND_EQUATION_MAX:    u64 = 0x0000000040000000_u64;
pub const BGFX_STATE_BLEND_EQUATION_MASK:   u64 = 0x00000003f0000000_u64;
pub const BGFX_STATE_BLEND_INDEPENDENT:     u64 = 0x0000000400000000_u64;
pub const BGFX_STATE_CULL_CW:               u64 = 0x0000001000000000_u64;
pub const BGFX_STATE_CULL_CCW:              u64 = 0x0000002000000000_u64;
pub const BGFX_STATE_CULL_MASK:             u64 = 0x0000003000000000_u64;
pub const BGFX_STATE_ALPHA_REF_MASK:        u64 = 0x0000ff0000000000_u64;
pub const BGFX_STATE_PT_TRISTRIP:           u64 = 0x0001000000000000_u64;
pub const BGFX_STATE_PT_LINES:              u64 = 0x0002000000000000_u64;
pub const BGFX_STATE_PT_LINESTRIP:          u64 = 0x0003000000000000_u64;
pub const BGFX_STATE_PT_POINTS:             u64 = 0x0004000000000000_u64;
pub const BGFX_STATE_PT_MASK:               u64 = 0x0007000000000000_u64;
pub const BGFX_STATE_POINT_SIZE_MASK:       u64 = 0x0ff0000000000000_u64;
pub const BGFX_STATE_MSAA:                  u64 = 0x1000000000000000_u64;
pub const BGFX_STATE_RESERVED_MASK:         u64 = 0xe000000000000000_u64;
pub const BGFX_STATE_NONE:                  u64 = 0x0000000000000000_u64;
pub const BGFX_STATE_MASK:                  u64 = 0xffffffffffffffff_u64;

pub const BGFX_STATE_DEFAULT:               u64 =
    (
        BGFX_STATE_RGB_WRITE |
        BGFX_STATE_ALPHA_WRITE |
        BGFX_STATE_DEPTH_TEST_LESS |
        BGFX_STATE_DEPTH_WRITE |
        BGFX_STATE_CULL_CW |
        BGFX_STATE_MSAA
	);

#[macro_export]
macro_rules! BGFX_STATE_ALPHA_REF {
    ($aref:expr) => ((($aref as u64) << bgfx_sys::BGFX_STATE_ALPHA_REF_SHIFT) & bgfx_sys::BGFX_STATE_ALPHA_REF_MASK)
}

#[macro_export]
macro_rules! BGFX_STATE_POINT_SIZE {
    ($size:expr) => ((($size as u64) << bgfx_sys::BGFX_STATE_POINT_SIZE_SHIFT) & bgfx_sys::BGFX_STATE_POINT_SIZE_MASK)
}

#[macro_export]
macro_rules! BGFX_STATE_BLEND_FUNC_SEPARATE {
    ($srcrgb:expr, $dstrgb:expr, $srca:expr, $dsta:expr) => (
        ($srcrgb as u64) | (($dstrgb as u64) << 4) | (($srca as u64) << 8) | (($dsta as u64) << 12)
    );
}

#[macro_export]
macro_rules! BGFX_STATE_BLEND_EQUATION_SEPARATE {
    ($rgb:expr, $a:expr) => (($rgb as u64) | (($a as u64) << 3))
}

#[macro_export]
macro_rules! BGFX_STATE_BLEND_FUNC {
    ($src:expr, $dst:expr) => (BGFX_STATE_BLEND_FUNC_SEPARATE!($src, $dst, $src, $dst))
}

#[macro_export]
macro_rules! BGFX_STATE_BLEND_EQUATION {
    ($equation:expr) => (BGFX_STATE_BLEND_EQUATION_SEPARATE!($equation, $equation))
}

pub const BGFX_STATE_BLEND_ADD:             u64 = (BGFX_STATE_BLEND_FUNC!(BGFX_STATE_BLEND_ONE,       BGFX_STATE_BLEND_ONE          ) );
pub const BGFX_STATE_BLEND_ALPHA:           u64 = (BGFX_STATE_BLEND_FUNC!(BGFX_STATE_BLEND_SRC_ALPHA, BGFX_STATE_BLEND_INV_SRC_ALPHA) );
pub const BGFX_STATE_BLEND_DARKEN:          u64 = (BGFX_STATE_BLEND_FUNC!(BGFX_STATE_BLEND_ONE,       BGFX_STATE_BLEND_ONE          ) | BGFX_STATE_BLEND_EQUATION!(BGFX_STATE_BLEND_EQUATION_MIN) );
pub const BGFX_STATE_BLEND_LIGHTEN:         u64 = (BGFX_STATE_BLEND_FUNC!(BGFX_STATE_BLEND_ONE,       BGFX_STATE_BLEND_ONE          ) | BGFX_STATE_BLEND_EQUATION!(BGFX_STATE_BLEND_EQUATION_MAX) );
pub const BGFX_STATE_BLEND_MULTIPLY:        u64 = (BGFX_STATE_BLEND_FUNC!(BGFX_STATE_BLEND_DST_COLOR, BGFX_STATE_BLEND_ZERO         ) );
pub const BGFX_STATE_BLEND_NORMAL:          u64 = (BGFX_STATE_BLEND_FUNC!(BGFX_STATE_BLEND_ONE,       BGFX_STATE_BLEND_INV_SRC_ALPHA) );
pub const BGFX_STATE_BLEND_SCREEN:          u64 = (BGFX_STATE_BLEND_FUNC!(BGFX_STATE_BLEND_ONE,       BGFX_STATE_BLEND_INV_SRC_COLOR) );
pub const BGFX_STATE_BLEND_LINEAR_BURN:     u64 = (BGFX_STATE_BLEND_FUNC!(BGFX_STATE_BLEND_DST_COLOR, BGFX_STATE_BLEND_INV_DST_COLOR) | BGFX_STATE_BLEND_EQUATION!(BGFX_STATE_BLEND_EQUATION_SUB) );

#[macro_export]
macro_rules! BGFX_STATE_BLEND_FUNC_RT_x {
    ($src:expr, $dst:expr) => (
        (($src >> bgfx_sys::BGFX_STATE_BLEND_SHIFT) as u32) | ((($dst >> bgfx_sys::BGFX_STATE_BLEND_SHIFT) as u32) << 4)
    );
}

#[macro_export]
macro_rules! BGFX_STATE_BLEND_FUNC_RT_xE {
    ($src:expr, $dst:expr, $equation:expr) => (
        BGFX_STATE_BLEND_FUNC_RT_x!($src, $dst) | ((($equation >> bgfx_sys::BGFX_STATE_BLEND_EQUATION_SHIFT) as u32) << 8)
    );
}

#[macro_export]
macro_rules! BGFX_STATE_BLEND_FUNC_RT_1 {
    ($src:expr, $dst:expr) => (BGFX_STATE_BLEND_FUNC_RT_x!($src, $dst))
}

#[macro_export]
macro_rules! BGFX_STATE_BLEND_FUNC_RT_2 {
    ($src:expr, $dst:expr) => (BGFX_STATE_BLEND_FUNC_RT_x!($src, $dst) << 11)
}

#[macro_export]
macro_rules! BGFX_STATE_BLEND_FUNC_RT_3 {
    ($src:expr, $dst:expr) => (BGFX_STATE_BLEND_FUNC_RT_x!($src, $dst) << 22)
}

#[macro_export]
macro_rules! BGFX_STATE_BLEND_FUNC_RT_1E {
    ($src:expr, $dst:expr, $equation:expr) => (BGFX_STATE_BLEND_FUNC_RT_xE!($src, $dst, $equation))
}

#[macro_export]
macro_rules! BGFX_STATE_BLEND_FUNC_RT_2E {
    ($src:expr, $dst:expr, $equation:expr) => (BGFX_STATE_BLEND_FUNC_RT_xE!($src, $dst, $equation) << 11)
}

#[macro_export]
macro_rules! BGFX_STATE_BLEND_FUNC_RT_3E {
    ($src:expr, $dst:expr, $equation:expr) => (BGFX_STATE_BLEND_FUNC_RT_xE!($src, $dst, $equation) << 22)
}

// ---------------

impl fmt::Debug for bgfx_caps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Caps {{ rendererType: {}, supported: 0x{:x}, vendorId: 0x{:x}, deviceId: {}, homogeneousDepth: {}, originBottomLeft: {}, numGPUs: {}, limits: {:?}, formats: [ {}] }}",
        self.rendererType,
        self.supported,
        self.vendorId,
        self.deviceId,
        self.homogeneousDepth,
        self.originBottomLeft,
        self.numGPUs,
        self.limits,
        self.formats.iter().fold(String::new(), |acc, &v| { format!("{}0x{:x}, ", acc, v)}))
    }
}

