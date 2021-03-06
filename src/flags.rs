// Copyright (c) 2015-2016, Johan Sköld.
// License: http://opensource.org/licenses/ISC

extern crate bgfx_sys;

bitflags! {
    pub flags StateFlags: u64 {
        const STATE_RGB_WRITE = bgfx_sys::BGFX_STATE_RGB_WRITE,
        const STATE_ALPHA_WRITE = bgfx_sys::BGFX_STATE_ALPHA_WRITE,
        const STATE_DEPTH_WRITE = bgfx_sys::BGFX_STATE_DEPTH_WRITE,
        const STATE_DEPTH_TEST_LESS = bgfx_sys::BGFX_STATE_DEPTH_TEST_LESS,
        const STATE_DEPTH_TEST_LEQUAL = bgfx_sys::BGFX_STATE_DEPTH_TEST_LEQUAL,
        const STATE_DEPTH_TEST_EQUAL = bgfx_sys::BGFX_STATE_DEPTH_TEST_EQUAL,
        const STATE_DEPTH_TEST_GEQUAL = bgfx_sys::BGFX_STATE_DEPTH_TEST_GEQUAL,
        const STATE_DEPTH_TEST_GREATER = bgfx_sys::BGFX_STATE_DEPTH_TEST_GREATER,
        const STATE_DEPTH_TEST_NOTEQUAL = bgfx_sys::BGFX_STATE_DEPTH_TEST_NOTEQUAL,
        const STATE_DEPTH_TEST_NEVER = bgfx_sys::BGFX_STATE_DEPTH_TEST_NEVER,
        const STATE_DEPTH_TEST_ALWAYS = bgfx_sys::BGFX_STATE_DEPTH_TEST_ALWAYS,
        const STATE_DEPTH_TEST_MASK = bgfx_sys::BGFX_STATE_DEPTH_TEST_MASK,
        const STATE_BLEND_ZERO = bgfx_sys::BGFX_STATE_BLEND_ZERO,
        const STATE_BLEND_ONE = bgfx_sys::BGFX_STATE_BLEND_ONE,
        const STATE_BLEND_SRC_COLOR = bgfx_sys::BGFX_STATE_BLEND_SRC_COLOR,
        const STATE_BLEND_INV_SRC_COLOR = bgfx_sys::BGFX_STATE_BLEND_INV_SRC_COLOR,
        const STATE_BLEND_SRC_ALPHA = bgfx_sys::BGFX_STATE_BLEND_SRC_ALPHA,
        const STATE_BLEND_INV_SRC_ALPHA = bgfx_sys::BGFX_STATE_BLEND_INV_SRC_ALPHA,
        const STATE_BLEND_DST_ALPHA = bgfx_sys::BGFX_STATE_BLEND_DST_ALPHA,
        const STATE_BLEND_INV_DST_ALPHA = bgfx_sys::BGFX_STATE_BLEND_INV_DST_ALPHA,
        const STATE_BLEND_DST_COLOR = bgfx_sys::BGFX_STATE_BLEND_DST_COLOR,
        const STATE_BLEND_INV_DST_COLOR = bgfx_sys::BGFX_STATE_BLEND_INV_DST_COLOR,
        const STATE_BLEND_SRC_ALPHA_SAT = bgfx_sys::BGFX_STATE_BLEND_SRC_ALPHA_SAT,
        const STATE_BLEND_FACTOR = bgfx_sys::BGFX_STATE_BLEND_FACTOR,
        const STATE_BLEND_INV_FACTOR = bgfx_sys::BGFX_STATE_BLEND_INV_FACTOR,
        const STATE_BLEND_MASK = bgfx_sys::BGFX_STATE_BLEND_MASK,
        const STATE_BLEND_EQUATION_ADD = bgfx_sys::BGFX_STATE_BLEND_EQUATION_ADD,
        const STATE_BLEND_EQUATION_SUB = bgfx_sys::BGFX_STATE_BLEND_EQUATION_SUB,
        const STATE_BLEND_EQUATION_REVSUB = bgfx_sys::BGFX_STATE_BLEND_EQUATION_REVSUB,
        const STATE_BLEND_EQUATION_MIN = bgfx_sys::BGFX_STATE_BLEND_EQUATION_MIN,
        const STATE_BLEND_EQUATION_MAX = bgfx_sys::BGFX_STATE_BLEND_EQUATION_MAX,
        const STATE_BLEND_EQUATION_MASK = bgfx_sys::BGFX_STATE_BLEND_EQUATION_MASK,
        const STATE_BLEND_INDEPENDENT = bgfx_sys::BGFX_STATE_BLEND_INDEPENDENT,
        const STATE_CULL_CW = bgfx_sys::BGFX_STATE_CULL_CW,
        const STATE_CULL_CCW = bgfx_sys::BGFX_STATE_CULL_CCW,
        const STATE_CULL_MASK = bgfx_sys::BGFX_STATE_CULL_MASK,
        const STATE_ALPHA_REF_MASK = bgfx_sys::BGFX_STATE_ALPHA_REF_MASK,
        const STATE_PT_TRISTRIP = bgfx_sys::BGFX_STATE_PT_TRISTRIP,
        const STATE_PT_LINES = bgfx_sys::BGFX_STATE_PT_LINES,
        const STATE_PT_LINESTRIP = bgfx_sys::BGFX_STATE_PT_LINESTRIP,
        const STATE_PT_POINTS = bgfx_sys::BGFX_STATE_PT_POINTS,
        const STATE_PT_MASK = bgfx_sys::BGFX_STATE_PT_MASK,
        const STATE_POINT_SIZE_MASK = bgfx_sys::BGFX_STATE_POINT_SIZE_MASK,
        const STATE_MSAA = bgfx_sys::BGFX_STATE_MSAA,
        const STATE_RESERVED_MASK = bgfx_sys::BGFX_STATE_RESERVED_MASK,
        const STATE_NONE = bgfx_sys::BGFX_STATE_NONE,
        const STATE_MASK = bgfx_sys::BGFX_STATE_MASK,
        const STATE_DEFAULT = bgfx_sys::BGFX_STATE_DEFAULT,
        const STATE_BLEND_ADD = bgfx_sys::BGFX_STATE_BLEND_ADD,
        const STATE_BLEND_ALPHA = bgfx_sys::BGFX_STATE_BLEND_ALPHA,
        const STATE_BLEND_DARKEN = bgfx_sys::BGFX_STATE_BLEND_DARKEN,
        const STATE_BLEND_LIGHTEN = bgfx_sys::BGFX_STATE_BLEND_LIGHTEN,
        const STATE_BLEND_MULTIPLY = bgfx_sys::BGFX_STATE_BLEND_MULTIPLY,
        const STATE_BLEND_NORMAL = bgfx_sys::BGFX_STATE_BLEND_NORMAL,
        const STATE_BLEND_SCREEN = bgfx_sys::BGFX_STATE_BLEND_SCREEN,
        const STATE_BLEND_LINEAR_BURN = bgfx_sys::BGFX_STATE_BLEND_LINEAR_BURN,
    }
}

pub const STATE_DEPTH_TEST_SHIFT: usize = bgfx_sys::BGFX_STATE_DEPTH_TEST_SHIFT as usize;
pub const STATE_BLEND_SHIFT: usize = bgfx_sys::BGFX_STATE_BLEND_SHIFT as usize;
pub const STATE_BLEND_EQUATION_SHIFT: usize = bgfx_sys::BGFX_STATE_BLEND_EQUATION_SHIFT as usize;
pub const STATE_CULL_SHIFT: usize = bgfx_sys::BGFX_STATE_CULL_SHIFT as usize;
pub const STATE_ALPHA_REF_SHIFT: usize = bgfx_sys::BGFX_STATE_ALPHA_REF_SHIFT as usize;
pub const STATE_PT_SHIFT: usize = bgfx_sys::BGFX_STATE_PT_SHIFT as usize;
pub const STATE_POINT_SIZE_SHIFT: usize = bgfx_sys::BGFX_STATE_POINT_SIZE_SHIFT as usize;

impl Default for StateFlags {
    #[inline]
    fn default() -> StateFlags {
        STATE_DEFAULT
    }
}

#[inline]
pub fn state_alpha_ref(aref: u8) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_ALPHA_REF!(aref)).unwrap()
}

#[inline]
pub fn state_point_size(size: u8) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_POINT_SIZE!(size)).unwrap()
}

#[inline]
pub fn state_blend_func_separate(srcrgb: u32, dstrgb: u32, srca: u8, dsta: u8) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_SEPARATE!(srcrgb, dstrgb, srca, dsta)).unwrap()
}

#[inline]
pub fn state_blend_equation_separate(rgb: u32, a: u8) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_EQUATION_SEPARATE!(rgb, a)).unwrap()
}

#[inline]
pub fn state_blend_func(src: u32, dst: u32) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_SEPARATE!(src, dst, src, dst)).unwrap()
}

#[inline]
pub fn state_blend_equation(equation: u32) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_EQUATION_SEPARATE!(equation, equation)).unwrap()
}

#[inline]
pub fn state_blend_func_rt_x(src: u32, dst: u32) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_RT_x!(src, dst) as u64).unwrap()
}

#[inline]
pub fn state_blend_func_rt_xe(src: u32, dst: u32, equation: u64) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_RT_xE!(src, dst, equation) as u64).unwrap()
}

#[inline]
pub fn state_blend_func_rt_1(src: u32, dst: u32) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_RT_1!(src, dst) as u64).unwrap()
}

#[inline]
pub fn state_blend_func_rt_2(src: u32, dst: u32) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_RT_2!(src, dst) as u64).unwrap()
}

#[inline]
pub fn state_blend_func_rt_3(src: u32, dst: u32) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_RT_3!(src, dst) as u64).unwrap()
}

#[inline]
pub fn state_blend_func_rt_1e(src: u32, dst: u32, equation: u64) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_RT_1E!(src, dst, equation) as u64).unwrap()
}

#[inline]
pub fn state_blend_func_rt_2e(src: u32, dst: u32, equation: u64) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_RT_2E!(src, dst, equation) as u64).unwrap()
}

#[inline]
pub fn state_blend_func_rt_3e(src: u32, dst: u32, equation: u64) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_RT_3E!(src, dst, equation) as u64).unwrap()
}

bitflags! {
    pub flags BufferFlags: u16 {
        const BUFFER_NONE = bgfx_sys::BGFX_BUFFER_NONE,
        const BUFFER_COMPUTE_FORMAT_8X1 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_8X1,
        const BUFFER_COMPUTE_FORMAT_8X2 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_8X2,
        const BUFFER_COMPUTE_FORMAT_8X4 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_8X4,
        const BUFFER_COMPUTE_FORMAT_16X1 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_16X1,
        const BUFFER_COMPUTE_FORMAT_16X2 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_16X2,
        const BUFFER_COMPUTE_FORMAT_16X4 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_16X4,
        const BUFFER_COMPUTE_FORMAT_32X1 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_32X1,
        const BUFFER_COMPUTE_FORMAT_32X2 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_32X2,
        const BUFFER_COMPUTE_FORMAT_32X4 = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_32X4,
        const BUFFER_COMPUTE_FORMAT_MASK = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_MASK,
        const BUFFER_COMPUTE_TYPE_UINT = bgfx_sys::BGFX_BUFFER_COMPUTE_TYPE_UINT,
        const BUFFER_COMPUTE_TYPE_INT = bgfx_sys::BGFX_BUFFER_COMPUTE_TYPE_INT,
        const BUFFER_COMPUTE_TYPE_FLOAT = bgfx_sys::BGFX_BUFFER_COMPUTE_TYPE_FLOAT,
        const BUFFER_COMPUTE_TYPE_MASK = bgfx_sys::BGFX_BUFFER_COMPUTE_TYPE_MASK,
        const BUFFER_COMPUTE_READ = bgfx_sys::BGFX_BUFFER_COMPUTE_READ,
        const BUFFER_COMPUTE_WRITE = bgfx_sys::BGFX_BUFFER_COMPUTE_WRITE,
        const BUFFER_DRAW_INDIRECT = bgfx_sys::BGFX_BUFFER_DRAW_INDIRECT,
        const BUFFER_ALLOW_RESIZE = bgfx_sys::BGFX_BUFFER_ALLOW_RESIZE,
        const BUFFER_INDEX32 = bgfx_sys::BGFX_BUFFER_INDEX32,
        const BUFFER_COMPUTE_READ_WRITE = bgfx_sys::BGFX_BUFFER_COMPUTE_READ_WRITE,
    }
}

pub const BUFFER_COMPUTE_FORMAT_SHIFT: usize = bgfx_sys::BGFX_BUFFER_COMPUTE_FORMAT_SHIFT as usize;
pub const BUFFER_COMPUTE_TYPE_SHIFT: usize = bgfx_sys::BGFX_BUFFER_COMPUTE_TYPE_SHIFT as usize;

impl Default for BufferFlags {
    #[inline]
    fn default() -> BufferFlags {
        BUFFER_NONE
    }
}

bitflags! {
    pub flags ClearFlags: u16 {
        const CLEAR_NONE = bgfx_sys::BGFX_CLEAR_NONE,
        const CLEAR_COLOR = bgfx_sys::BGFX_CLEAR_COLOR,
        const CLEAR_DEPTH = bgfx_sys::BGFX_CLEAR_DEPTH,
        const CLEAR_STENCIL = bgfx_sys::BGFX_CLEAR_STENCIL,
        const CLEAR_DISCARD_COLOR_0 = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_0,
        const CLEAR_DISCARD_COLOR_1 = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_1,
        const CLEAR_DISCARD_COLOR_2 = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_2,
        const CLEAR_DISCARD_COLOR_3 = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_3,
        const CLEAR_DISCARD_COLOR_4 = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_4,
        const CLEAR_DISCARD_COLOR_5 = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_5,
        const CLEAR_DISCARD_COLOR_6 = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_6,
        const CLEAR_DISCARD_COLOR_7 = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_7,
        const CLEAR_DISCARD_DEPTH = bgfx_sys::BGFX_CLEAR_DISCARD_DEPTH,
        const CLEAR_DISCARD_STENCIL = bgfx_sys::BGFX_CLEAR_DISCARD_STENCIL,
        const CLEAR_DISCARD_COLOR_MASK = bgfx_sys::BGFX_CLEAR_DISCARD_COLOR_MASK,
        const CLEAR_DISCARD_MASK = bgfx_sys::BGFX_CLEAR_DISCARD_MASK,
    }
}

impl Default for ClearFlags {
    #[inline]
    fn default() -> ClearFlags {
        CLEAR_NONE
    }
}

bitflags! {
    pub flags DebugFlags: u32 {
        const DEBUG_NONE = bgfx_sys::BGFX_DEBUG_NONE,
        const DEBUG_WIREFRAME = bgfx_sys::BGFX_DEBUG_WIREFRAME,
        const DEBUG_IFH = bgfx_sys::BGFX_DEBUG_IFH,
        const DEBUG_STATS = bgfx_sys::BGFX_DEBUG_STATS,
        const DEBUG_TEXT = bgfx_sys::BGFX_DEBUG_TEXT,
    }
}

impl Default for DebugFlags {
    #[inline]
    fn default() -> DebugFlags {
        DEBUG_NONE
    }
}

bitflags! {
    pub flags ResetFlags: u32 {
        const RESET_NONE = bgfx_sys::BGFX_RESET_NONE,
        const RESET_FULLSCREEN = bgfx_sys::BGFX_RESET_FULLSCREEN,
        const RESET_FULLSCREEN_SHIFT = bgfx_sys::BGFX_RESET_FULLSCREEN_SHIFT,
        const RESET_FULLSCREEN_MASK = bgfx_sys::BGFX_RESET_FULLSCREEN_MASK,
        const RESET_MSAA_X2 = bgfx_sys::BGFX_RESET_MSAA_X2,
        const RESET_MSAA_X4 = bgfx_sys::BGFX_RESET_MSAA_X4,
        const RESET_MSAA_X8 = bgfx_sys::BGFX_RESET_MSAA_X8,
        const RESET_MSAA_X16 = bgfx_sys::BGFX_RESET_MSAA_X16,
        const RESET_MSAA_SHIFT = bgfx_sys::BGFX_RESET_MSAA_SHIFT,
        const RESET_MSAA_MASK = bgfx_sys::BGFX_RESET_MSAA_MASK,
        const RESET_VSYNC = bgfx_sys::BGFX_RESET_VSYNC,
        const RESET_MAXANISOTROPY = bgfx_sys::BGFX_RESET_MAXANISOTROPY,
        const RESET_CAPTURE = bgfx_sys::BGFX_RESET_CAPTURE,
        const RESET_HMD = bgfx_sys::BGFX_RESET_HMD,
        const RESET_HMD_DEBUG = bgfx_sys::BGFX_RESET_HMD_DEBUG,
        const RESET_HMD_RECENTER = bgfx_sys::BGFX_RESET_HMD_RECENTER,
        const RESET_FLUSH_AFTER_RENDER = bgfx_sys::BGFX_RESET_FLUSH_AFTER_RENDER,
        const RESET_FLIP_AFTER_RENDER = bgfx_sys::BGFX_RESET_FLIP_AFTER_RENDER,
        const RESET_SRGB_BACKBUFFER = bgfx_sys::BGFX_RESET_SRGB_BACKBUFFER,
        const RESET_HIDPI = bgfx_sys::BGFX_RESET_HIDPI,
        const RESET_DEPTH_CLAMP = bgfx_sys::BGFX_RESET_DEPTH_CLAMP,
        const RESET_SUSPEND = bgfx_sys::BGFX_RESET_SUSPEND,
    }
}

impl Default for ResetFlags {
    #[inline]
    fn default() -> ResetFlags {
        RESET_NONE
    }
}

bitflags! {
    pub flags TextureFlags: u32 {

        const TEXTURE_NONE = bgfx_sys::BGFX_TEXTURE_NONE,
        const TEXTURE_U_MIRROR = bgfx_sys::BGFX_TEXTURE_U_MIRROR,
        const TEXTURE_U_CLAMP = bgfx_sys::BGFX_TEXTURE_U_CLAMP,
        const TEXTURE_U_BORDER = bgfx_sys::BGFX_TEXTURE_U_BORDER,
        const TEXTURE_U_MASK = bgfx_sys::BGFX_TEXTURE_U_MASK,
        const TEXTURE_V_MIRROR = bgfx_sys::BGFX_TEXTURE_V_MIRROR,
        const TEXTURE_V_CLAMP = bgfx_sys::BGFX_TEXTURE_V_CLAMP,
        const TEXTURE_V_BORDER = bgfx_sys::BGFX_TEXTURE_V_BORDER,
        const TEXTURE_V_MASK = bgfx_sys::BGFX_TEXTURE_V_MASK,
        const TEXTURE_W_MIRROR = bgfx_sys::BGFX_TEXTURE_W_MIRROR,
        const TEXTURE_W_CLAMP = bgfx_sys::BGFX_TEXTURE_W_CLAMP,
        const TEXTURE_W_BORDER = bgfx_sys::BGFX_TEXTURE_W_BORDER,
        const TEXTURE_W_MASK = bgfx_sys::BGFX_TEXTURE_W_MASK,
        const TEXTURE_MIN_POINT = bgfx_sys::BGFX_TEXTURE_MIN_POINT,
        const TEXTURE_MIN_ANISOTROPIC = bgfx_sys::BGFX_TEXTURE_MIN_ANISOTROPIC,
        const TEXTURE_MIN_MASK = bgfx_sys::BGFX_TEXTURE_MIN_MASK,
        const TEXTURE_MAG_POINT = bgfx_sys::BGFX_TEXTURE_MAG_POINT,
        const TEXTURE_MAG_ANISOTROPIC = bgfx_sys::BGFX_TEXTURE_MAG_ANISOTROPIC,
        const TEXTURE_MAG_MASK = bgfx_sys::BGFX_TEXTURE_MAG_MASK,
        const TEXTURE_MIP_POINT = bgfx_sys::BGFX_TEXTURE_MIP_POINT,
        const TEXTURE_MIP_MASK = bgfx_sys::BGFX_TEXTURE_MIP_MASK,
        const TEXTURE_MSAA_SAMPLE = bgfx_sys::BGFX_TEXTURE_MSAA_SAMPLE,
        const TEXTURE_RT = bgfx_sys::BGFX_TEXTURE_RT,
        const TEXTURE_RT_MSAA_X2 = bgfx_sys::BGFX_TEXTURE_RT_MSAA_X2,
        const TEXTURE_RT_MSAA_X4 = bgfx_sys::BGFX_TEXTURE_RT_MSAA_X4,
        const TEXTURE_RT_MSAA_X8 = bgfx_sys::BGFX_TEXTURE_RT_MSAA_X8,
        const TEXTURE_RT_MSAA_X16 = bgfx_sys::BGFX_TEXTURE_RT_MSAA_X16,
        const TEXTURE_RT_MSAA_MASK = bgfx_sys::BGFX_TEXTURE_RT_MSAA_MASK,
        const TEXTURE_RT_WRITE_ONLY = bgfx_sys::BGFX_TEXTURE_RT_WRITE_ONLY,
        const TEXTURE_RT_MASK = bgfx_sys::BGFX_TEXTURE_RT_MASK,
        const TEXTURE_COMPARE_LESS = bgfx_sys::BGFX_TEXTURE_COMPARE_LESS,
        const TEXTURE_COMPARE_LEQUAL = bgfx_sys::BGFX_TEXTURE_COMPARE_LEQUAL,
        const TEXTURE_COMPARE_EQUAL = bgfx_sys::BGFX_TEXTURE_COMPARE_EQUAL,
        const TEXTURE_COMPARE_GEQUAL = bgfx_sys::BGFX_TEXTURE_COMPARE_GEQUAL,
        const TEXTURE_COMPARE_GREATER = bgfx_sys::BGFX_TEXTURE_COMPARE_GREATER,
        const TEXTURE_COMPARE_NOTEQUAL = bgfx_sys::BGFX_TEXTURE_COMPARE_NOTEQUAL,
        const TEXTURE_COMPARE_NEVER = bgfx_sys::BGFX_TEXTURE_COMPARE_NEVER,
        const TEXTURE_COMPARE_ALWAYS = bgfx_sys::BGFX_TEXTURE_COMPARE_ALWAYS,
        const TEXTURE_COMPARE_MASK = bgfx_sys::BGFX_TEXTURE_COMPARE_MASK,
        const TEXTURE_COMPUTE_WRITE = bgfx_sys::BGFX_TEXTURE_COMPUTE_WRITE,
        const TEXTURE_SRGB = bgfx_sys::BGFX_TEXTURE_SRGB,
        const TEXTURE_BLIT_DST = bgfx_sys::BGFX_TEXTURE_BLIT_DST,
        const TEXTURE_READ_BACK = bgfx_sys::BGFX_TEXTURE_READ_BACK,
        const TEXTURE_BORDER_COLOR_MASK = bgfx_sys::BGFX_TEXTURE_BORDER_COLOR_MASK,
        const TEXTURE_RESERVED_MASK = bgfx_sys::BGFX_TEXTURE_RESERVED_MASK,
        const TEXTURE_SAMPLER_BITS_MASK = bgfx_sys::BGFX_TEXTURE_SAMPLER_BITS_MASK,
    }
}

#[inline]
pub fn texture_border_color(color: u32) -> TextureFlags {
    TextureFlags::from_bits(BGFX_TEXTURE_BORDER_COLOR!(color)).unwrap()
}

impl Default for TextureFlags {
    #[inline]
    fn default() -> Self {
        TEXTURE_NONE
    }
}

/// Bgfx Caps

bitflags! {
    pub flags CapsFlags: u64 {

        const CAPS_NONE = 0,
        const CAPS_ALPHA_TO_COVERAGE = bgfx_sys::BGFX_CAPS_ALPHA_TO_COVERAGE,
        const CAPS_BLEND_INDEPENDENT = bgfx_sys::BGFX_CAPS_BLEND_INDEPENDENT,
        const CAPS_COMPUTE = bgfx_sys::BGFX_CAPS_COMPUTE,
        const CAPS_CONSERVATIVE_RASTER = bgfx_sys::BGFX_CAPS_CONSERVATIVE_RASTER,
        const CAPS_DRAW_INDIRECT = bgfx_sys::BGFX_CAPS_DRAW_INDIRECT,
        const CAPS_FRAGMENT_DEPTH = bgfx_sys::BGFX_CAPS_FRAGMENT_DEPTH,
        const CAPS_FRAGMENT_ORDERING = bgfx_sys::BGFX_CAPS_FRAGMENT_ORDERING,
        const CAPS_GRAPHICS_DEBUGGER = bgfx_sys::BGFX_CAPS_GRAPHICS_DEBUGGER,
        const CAPS_HIDPI = bgfx_sys::BGFX_CAPS_HIDPI,
        const CAPS_HMD = bgfx_sys::BGFX_CAPS_HMD,
        const CAPS_INDEX32 = bgfx_sys::BGFX_CAPS_INDEX32,
        const CAPS_INSTANCING = bgfx_sys::BGFX_CAPS_INSTANCING,
        const CAPS_OCCLUSION_QUERY = bgfx_sys::BGFX_CAPS_OCCLUSION_QUERY,
        const CAPS_RENDERER_MULTITHREADED = bgfx_sys::BGFX_CAPS_RENDERER_MULTITHREADED,
        const CAPS_SWAP_CHAIN = bgfx_sys::BGFX_CAPS_SWAP_CHAIN,
        const CAPS_TEXTURE_2D_ARRAY = bgfx_sys::BGFX_CAPS_TEXTURE_2D_ARRAY,
        const CAPS_TEXTURE_3D = bgfx_sys::BGFX_CAPS_TEXTURE_3D,
        const CAPS_TEXTURE_BLIT = bgfx_sys::BGFX_CAPS_TEXTURE_BLIT,
        const CAPS_TEXTURE_COMPARE_ALL = bgfx_sys::BGFX_CAPS_TEXTURE_COMPARE_ALL,
        const CAPS_TEXTURE_COMPARE_LEQUAL = bgfx_sys::BGFX_CAPS_TEXTURE_COMPARE_LEQUAL,
        const CAPS_TEXTURE_CUBE_ARRAY = bgfx_sys::BGFX_CAPS_TEXTURE_CUBE_ARRAY,
        const CAPS_TEXTURE_READ_BACK = bgfx_sys::BGFX_CAPS_TEXTURE_READ_BACK,
        const CAPS_VERTEX_ATTRIB_HALF = bgfx_sys::BGFX_CAPS_VERTEX_ATTRIB_HALF,
        const CAPS_VERTEX_ATTRIB_UINT10 = bgfx_sys::BGFX_CAPS_VERTEX_ATTRIB_UINT10,
    }
}

impl Default for CapsFlags {
    #[inline]
    fn default() -> Self {
        CAPS_NONE
    }
}

/// Texture Caps

bitflags! {
    pub flags TextureCapsFlags: u16 {
        const CAPS_FORMAT_TEXTURE_NONE = bgfx_sys::BGFX_CAPS_FORMAT_TEXTURE_NONE,
        const CAPS_FORMAT_TEXTURE_2D = bgfx_sys::BGFX_CAPS_FORMAT_TEXTURE_2D,
        const CAPS_FORMAT_TEXTURE_2D_SRGB = bgfx_sys::BGFX_CAPS_FORMAT_TEXTURE_2D_SRGB,
        const CAPS_FORMAT_TEXTURE_2D_EMULATED = bgfx_sys::BGFX_CAPS_FORMAT_TEXTURE_2D_EMULATED,
        const CAPS_FORMAT_TEXTURE_3D = bgfx_sys::BGFX_CAPS_FORMAT_TEXTURE_3D,
        const CAPS_FORMAT_TEXTURE_3D_SRGB = bgfx_sys::BGFX_CAPS_FORMAT_TEXTURE_3D_SRGB,
        const CAPS_FORMAT_TEXTURE_3D_EMULATED = bgfx_sys::BGFX_CAPS_FORMAT_TEXTURE_3D_EMULATED,
        const CAPS_FORMAT_TEXTURE_CUBE = bgfx_sys::BGFX_CAPS_FORMAT_TEXTURE_CUBE,
        const CAPS_FORMAT_TEXTURE_CUBE_SRGB = bgfx_sys::BGFX_CAPS_FORMAT_TEXTURE_CUBE_SRGB,
        const CAPS_FORMAT_TEXTURE_CUBE_EMULATED = bgfx_sys::BGFX_CAPS_FORMAT_TEXTURE_CUBE_EMULATED,
        const CAPS_FORMAT_TEXTURE_VERTEX = bgfx_sys::BGFX_CAPS_FORMAT_TEXTURE_VERTEX,
        const CAPS_FORMAT_TEXTURE_IMAGE = bgfx_sys::BGFX_CAPS_FORMAT_TEXTURE_IMAGE,
        const CAPS_FORMAT_TEXTURE_FRAMEBUFFER = bgfx_sys::BGFX_CAPS_FORMAT_TEXTURE_FRAMEBUFFER,
        const CAPS_FORMAT_TEXTURE_FRAMEBUFFER_MSAA = bgfx_sys::BGFX_CAPS_FORMAT_TEXTURE_FRAMEBUFFER_MSAA,
        const CAPS_FORMAT_TEXTURE_MSAA = bgfx_sys::BGFX_CAPS_FORMAT_TEXTURE_MSAA,
        const CAPS_FORMAT_TEXTURE_MIP_AUTOGEN = bgfx_sys::BGFX_CAPS_FORMAT_TEXTURE_MIP_AUTOGEN,
    }
}

impl Default for TextureCapsFlags {
    #[inline]
    fn default() -> Self {
        CAPS_FORMAT_TEXTURE_NONE
    }
}
