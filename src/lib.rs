// Copyright (c) 2015-2016, Johan Sköld.
// License: http://opensource.org/licenses/ISC

//! Rust wrapper around [bgfx].
//!
//! Before using this crate, ensure that you fullfill the build requirements for bgfx, as outlined
//! in its [documentation][bgfx building]. If you are compiling for an `msvc` target, make sure to
//! build this crate in a developer command prompt.
//!
//! ## Limitations
//!
//! - So far, only Windows, Linux, and OSX are supported.
//! - Far from all bgfx functionality is exposed. As more examples get ported, more functionality
//!   will be as well.
//!
//! *This API is still unstable, and very likely to change.*
//!
//! ## Basic Usage
//!
//! Before this crate can be used, some platform data must be initialized. See [`PlatformData`].
//!
//! ```should_panic
//! bgfx::PlatformData::new()
//!     .context(std::ptr::null_mut())
//!     .display(std::ptr::null_mut())
//!     .window(std::ptr::null_mut())
//!     .apply()
//!     .expect("Could not set platform data");
//! ```
//!
//! Once the platform data has been initialized, a new thread should be spawned to act as the main
//! thread. This thread should call [`bgfx::init`] to initialize bgfx. The object returned by that
//! function should be used to access bgfx API calls.
//!
//! ```no_run
//! std::thread::spawn(|| {
//!     let bgfx = bgfx::init(bgfx::RendererType::Default, None, None)
//!         .expect("Failed to initialize bgfx");
//!     // ...
//! });
//! ```
//!
//! Finally, the real main thread should act as the render thread, and repeatedly be calling
//! [`bgfx::render_frame`].
//!
//! ```no_run
//! loop {
//!     // This is probably also where you will want to pump the window event queue.
//!     bgfx::render_frame();
//! }
//! ```
//!
//! See the examples for more in-depth usage.
//!
//! [bgfx]: https://github.com/bkaradzic/bgfx
//! [bgfx building]: https://bkaradzic.github.io/bgfx/build.html
//! [`bgfx::init`]: fn.init.html
//! [`bgfx::render_frame`]: fn.render_frame.html
//! [`PlatformData`]: struct.PlatformData.html

#[macro_use]
extern crate bgfx_sys;
#[macro_use]
extern crate bitflags;
extern crate libc;

use std::ffi;
use std::marker::PhantomData;
use std::mem;
use std::ptr;
use std::fmt;

pub mod flags;

pub use flags::*;


#[repr(u16)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum VendorId {
    /// Autoselect adapter.
    None = bgfx_sys::BGFX_PCI_ID_NONE,

    /// Software rasterizer.
    SoftwareRasterizer = bgfx_sys::BGFX_PCI_ID_SOFTWARE_RASTERIZER,

    /// AMD adapter.
    AMD = bgfx_sys::BGFX_PCI_ID_AMD,

    /// Intel adapter.
    Intel = bgfx_sys::BGFX_PCI_ID_INTEL,

    /// nVidia adapter.
    nVidia = bgfx_sys::BGFX_PCI_ID_NVIDIA,

    /// Microsoft DirectX Basic Render Driver
    MSBasicRender = 0x1414u16,
}

impl Default for VendorId {
    fn default() -> Self { VendorId::None }
}

/// Renderer backend type.
#[repr(i32)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum RendererType {
    /// No rendering.
    Null = bgfx_sys::BGFX_RENDERER_TYPE_NOOP,

    /// Direct3D 9.0.
    Direct3D9 = bgfx_sys::BGFX_RENDERER_TYPE_DIRECT3D9,

    /// Direct3D 11.0.
    Direct3D11 = bgfx_sys::BGFX_RENDERER_TYPE_DIRECT3D11,

    /// Direct3D 12.0.
    Direct3D12 = bgfx_sys::BGFX_RENDERER_TYPE_DIRECT3D12,

    /// Metal.
    Metal = bgfx_sys::BGFX_RENDERER_TYPE_METAL,

    /// OpenGLES.
    OpenGLES = bgfx_sys::BGFX_RENDERER_TYPE_OPENGLES,

    /// OpenGL.
    OpenGL = bgfx_sys::BGFX_RENDERER_TYPE_OPENGL,

    /// Vulkan.
    Vulkan = bgfx_sys::BGFX_RENDERER_TYPE_VULKAN,

    /// Use the most platform appropriate renderer.
    Default = bgfx_sys::BGFX_RENDERER_TYPE_COUNT,
}

impl Default for RendererType {
    fn default() -> Self { RendererType::Default }
}

impl RendererType {

    fn from_i32(n: i32) -> Option<RendererType> {
        if n <= bgfx_sys::BGFX_RENDERER_TYPE_COUNT {
            Some(unsafe { mem::transmute(n) })
        } else {
            None
        }
    }

}

/// Texture formats.
#[repr(i32)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum TextureFormat {
    BC1 = bgfx_sys::BGFX_TEXTURE_FORMAT_BC1,
    BC2 = bgfx_sys::BGFX_TEXTURE_FORMAT_BC2,
    BC3 = bgfx_sys::BGFX_TEXTURE_FORMAT_BC3,
    BC4 = bgfx_sys::BGFX_TEXTURE_FORMAT_BC4,
    BC5 = bgfx_sys::BGFX_TEXTURE_FORMAT_BC5,
    BC6H = bgfx_sys::BGFX_TEXTURE_FORMAT_BC6H,
    BC7 = bgfx_sys::BGFX_TEXTURE_FORMAT_BC7,
    ETC1 = bgfx_sys::BGFX_TEXTURE_FORMAT_ETC1,
    ETC2 = bgfx_sys::BGFX_TEXTURE_FORMAT_ETC2,
    ETC2A = bgfx_sys::BGFX_TEXTURE_FORMAT_ETC2A,
    ETC2A1 = bgfx_sys::BGFX_TEXTURE_FORMAT_ETC2A1,
    PTC12 = bgfx_sys::BGFX_TEXTURE_FORMAT_PTC12,
    PTC14 = bgfx_sys::BGFX_TEXTURE_FORMAT_PTC14,
    PTC12A = bgfx_sys::BGFX_TEXTURE_FORMAT_PTC12A,
    PTC14A = bgfx_sys::BGFX_TEXTURE_FORMAT_PTC14A,
    PTC22 = bgfx_sys::BGFX_TEXTURE_FORMAT_PTC22,
    PTC24 = bgfx_sys::BGFX_TEXTURE_FORMAT_PTC24,
    Unknown = bgfx_sys::BGFX_TEXTURE_FORMAT_UNKNOWN,
    R1 = bgfx_sys::BGFX_TEXTURE_FORMAT_R1,
    A8 = bgfx_sys::BGFX_TEXTURE_FORMAT_A8,
    R8 = bgfx_sys::BGFX_TEXTURE_FORMAT_R8,
    R8I = bgfx_sys::BGFX_TEXTURE_FORMAT_R8I,
    R8U = bgfx_sys::BGFX_TEXTURE_FORMAT_R8U,
    R8S = bgfx_sys::BGFX_TEXTURE_FORMAT_R8S,
    R16 = bgfx_sys::BGFX_TEXTURE_FORMAT_R16,
    R16I = bgfx_sys::BGFX_TEXTURE_FORMAT_R16I,
    R16U = bgfx_sys::BGFX_TEXTURE_FORMAT_R16U,
    R16F = bgfx_sys::BGFX_TEXTURE_FORMAT_R16F,
    R16S = bgfx_sys::BGFX_TEXTURE_FORMAT_R16S,
    R32I = bgfx_sys::BGFX_TEXTURE_FORMAT_R32I,
    R32U = bgfx_sys::BGFX_TEXTURE_FORMAT_R32U,
    R32F = bgfx_sys::BGFX_TEXTURE_FORMAT_R32F,
    RG8 = bgfx_sys::BGFX_TEXTURE_FORMAT_RG8,
    RG8I = bgfx_sys::BGFX_TEXTURE_FORMAT_RG8I,
    RG8U = bgfx_sys::BGFX_TEXTURE_FORMAT_RG8U,
    RG8S = bgfx_sys::BGFX_TEXTURE_FORMAT_RG8S,
    RG16 = bgfx_sys::BGFX_TEXTURE_FORMAT_RG16,
    RG16I = bgfx_sys::BGFX_TEXTURE_FORMAT_RG16I,
    RG16U = bgfx_sys::BGFX_TEXTURE_FORMAT_RG16U,
    RG16F = bgfx_sys::BGFX_TEXTURE_FORMAT_RG16F,
    RG16S = bgfx_sys::BGFX_TEXTURE_FORMAT_RG16S,
    RG32I = bgfx_sys::BGFX_TEXTURE_FORMAT_RG32I,
    RG32U = bgfx_sys::BGFX_TEXTURE_FORMAT_RG32U,
    RG32F = bgfx_sys::BGFX_TEXTURE_FORMAT_RG32F,
    RGB8 = bgfx_sys::BGFX_TEXTURE_FORMAT_RGB8,
    RGB8I = bgfx_sys::BGFX_TEXTURE_FORMAT_RGB8I,
    RGB8U = bgfx_sys::BGFX_TEXTURE_FORMAT_RGB8U,
    RGB8S = bgfx_sys::BGFX_TEXTURE_FORMAT_RGB8S,
    RGB9E5F = bgfx_sys::BGFX_TEXTURE_FORMAT_RGB9E5F,
    BGRA8 = bgfx_sys::BGFX_TEXTURE_FORMAT_BGRA8,
    RGBA8 = bgfx_sys::BGFX_TEXTURE_FORMAT_RGBA8,
    RGBA8I = bgfx_sys::BGFX_TEXTURE_FORMAT_RGBA8I,
    RGBA8U = bgfx_sys::BGFX_TEXTURE_FORMAT_RGBA8U,
    RGBA8S = bgfx_sys::BGFX_TEXTURE_FORMAT_RGBA8S,
    RGBA16 = bgfx_sys::BGFX_TEXTURE_FORMAT_RGBA16,
    RGBA16I = bgfx_sys::BGFX_TEXTURE_FORMAT_RGBA16I,
    RGBA16U = bgfx_sys::BGFX_TEXTURE_FORMAT_RGBA16U,
    RGBA16F = bgfx_sys::BGFX_TEXTURE_FORMAT_RGBA16F,
    RGBA16S = bgfx_sys::BGFX_TEXTURE_FORMAT_RGBA16S,
    RGBA32I = bgfx_sys::BGFX_TEXTURE_FORMAT_RGBA32I,
    RGBA32U = bgfx_sys::BGFX_TEXTURE_FORMAT_RGBA32U,
    RGBA32F = bgfx_sys::BGFX_TEXTURE_FORMAT_RGBA32F,
    R5G6B5 = bgfx_sys::BGFX_TEXTURE_FORMAT_R5G6B5,
    RGBA4 = bgfx_sys::BGFX_TEXTURE_FORMAT_RGBA4,
    RGB5A1 = bgfx_sys::BGFX_TEXTURE_FORMAT_RGB5A1,
    RGB10A2 = bgfx_sys::BGFX_TEXTURE_FORMAT_RGB10A2,
    R11G11B10F = bgfx_sys::BGFX_TEXTURE_FORMAT_R11G11B10F,
    UnknownDepth = bgfx_sys::BGFX_TEXTURE_FORMAT_UNKNOWN_DEPTH,
    D16 = bgfx_sys::BGFX_TEXTURE_FORMAT_D16,
    D24 = bgfx_sys::BGFX_TEXTURE_FORMAT_D24,
    D24S8 = bgfx_sys::BGFX_TEXTURE_FORMAT_D24S8,
    D32 = bgfx_sys::BGFX_TEXTURE_FORMAT_D32,
    D16F = bgfx_sys::BGFX_TEXTURE_FORMAT_D16F,
    D24F = bgfx_sys::BGFX_TEXTURE_FORMAT_D24F,
    D32F = bgfx_sys::BGFX_TEXTURE_FORMAT_D32F,
    D0S8 = bgfx_sys::BGFX_TEXTURE_FORMAT_D0S8,
    Count = bgfx_sys::BGFX_TEXTURE_FORMAT_COUNT,
}

impl Default for TextureFormat {
    fn default() -> Self { TextureFormat::Unknown }
}

impl TextureFormat {

    fn from_i32(n: i32) -> Option<TextureFormat> {
        if n <= bgfx_sys::BGFX_TEXTURE_FORMAT_COUNT {
            Some(unsafe { mem::transmute(n) })
        } else {
            None
        }
    }

}

/// `render_frame()` results.
#[repr(i32)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum RenderFrame {
    /// No context is available. This usually means the main thread has exited.
    NoContext = bgfx_sys::BGFX_RENDER_FRAME_NO_CONTEXT,

    /// The render was performed.
    Render = bgfx_sys::BGFX_RENDER_FRAME_RENDER,

    /// The renderer is exiting.
    Exiting = bgfx_sys::BGFX_RENDER_FRAME_EXITING,
}

impl RenderFrame {

    fn from_i32(n: i32) -> Option<RenderFrame> {
        if n < bgfx_sys::BGFX_RENDER_FRAME_COUNT {
            Some(unsafe { mem::transmute(n) })
        } else {
            None
        }
    }

}

/// Vertex attribute.
#[repr(u32)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Attrib {
    /// Position.
    Position = bgfx_sys::BGFX_ATTRIB_POSITION as u32,

    /// Normal.
    Normal = bgfx_sys::BGFX_ATTRIB_NORMAL as u32,

    /// Tangent.
    Tangent = bgfx_sys::BGFX_ATTRIB_TANGENT as u32,

    /// Bitangent.
    Bitangent = bgfx_sys::BGFX_ATTRIB_BITANGENT as u32,

    /// Color 0.
    Color0 = bgfx_sys::BGFX_ATTRIB_COLOR0 as u32,

    /// Color 1.
    Color1 = bgfx_sys::BGFX_ATTRIB_COLOR1 as u32,

    /// Index list.
    Indices = bgfx_sys::BGFX_ATTRIB_INDICES as u32,

    /// Bone weight.
    Weight = bgfx_sys::BGFX_ATTRIB_WEIGHT as u32,

    /// Texture coordinate 0.
    TexCoord0 = bgfx_sys::BGFX_ATTRIB_TEXCOORD0 as u32,

    /// Texture coordinate 1.
    TexCoord1 = bgfx_sys::BGFX_ATTRIB_TEXCOORD1 as u32,

    /// Texture coordinate 2.
    TexCoord2 = bgfx_sys::BGFX_ATTRIB_TEXCOORD2 as u32,

    /// Texture coordinate 3.
    TexCoord3 = bgfx_sys::BGFX_ATTRIB_TEXCOORD3 as u32,

    /// Texture coordinate 4.
    TexCoord4 = bgfx_sys::BGFX_ATTRIB_TEXCOORD4 as u32,

    /// Texture coordinate 5.
    TexCoord5 = bgfx_sys::BGFX_ATTRIB_TEXCOORD5 as u32,

    /// Texture coordinate 6.
    TexCoord6 = bgfx_sys::BGFX_ATTRIB_TEXCOORD6 as u32,

    /// Texture coordinate 7.
    TexCoord7 = bgfx_sys::BGFX_ATTRIB_TEXCOORD7 as u32,
}

/// Vertex attribute type.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum AttribType {
    /// Unsigned 8-bit integer.
    ///
    /// If the parameter is `true`, the value will be normalized between 0 and 1.
    Uint8(bool),

    /// Signed 8-bit integer.
    ///
    /// If the parameter is `true`, the value will be normalized between 0 and 1.
    Int8(bool),

    /// Unsigned 10-bit integer.
    ///
    /// If the parameter is `true`, the value will be normalized between 0 and 1.
    Uint10(bool),

    /// Signed 10-bit integer.
    ///
    /// If the parameter is `true`, the value will be normalized between 0 and 1.
    Int10(bool),

    /// Unsigned 16-bit integer.
    ///
    /// If the parameter is `true`, the value will be normalized between 0 and 1.
    Uint16(bool),

    /// Signed 16-bit integer.
    ///
    /// If the parameter is `true`, the value will be normalized between 0 and 1.
    Int16(bool),

    /// 16-bit float.
    Half,

    /// 32-bit float.
    Float,
}

/// Uniform types.
#[repr(u32)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum UniformType {

    Int1 = bgfx_sys::BGFX_UNIFORM_TYPE_INT1 as u32,
    End = bgfx_sys::BGFX_UNIFORM_TYPE_END as u32,
    Vec4 = bgfx_sys::BGFX_UNIFORM_TYPE_VEC4 as u32,
    Mat3 = bgfx_sys::BGFX_UNIFORM_TYPE_MAT3 as u32,
    Mat4 = bgfx_sys::BGFX_UNIFORM_TYPE_MAT4 as u32,
}

/// bgfx error.
#[derive(Debug)]
pub enum BgfxError {
    /// An invalid display was provided in the platform data.
    InvalidDisplay,

    /// An invalid window was provided in the platform data.
    InvalidWindow,

    /// Initialization failed.
    InitFailed,
}

/// bgfx-managed buffer of memory.
///
/// It can be created by either copying existing data through [`copy(...)`], or by referencing
/// existing memory directly through [`reference(...)`].
///
/// [`copy(...)`]: #method.copy
/// [`reference(...)`]: #method.reference
pub struct Memory<'b> {
    handle: *const bgfx_sys::bgfx_memory_t,
    _phantom: PhantomData<&'b ()>,
}

impl<'b> Memory<'b> {

    /// Copies the source data into a new bgfx-managed buffer.
    ///
    /// **IMPORTANT:** If this buffer is never passed into a bgfx call, the memory will never be
    /// freed, and will leak.
    #[inline]
    pub fn copy<T>(_bgfx: &Bgfx, data: &[T]) -> Memory<'b> {
        unsafe {
            let handle = bgfx_sys::bgfx_copy(data.as_ptr() as *const ::std::os::raw::c_void,
                                             mem::size_of_val(data) as u32);
            Memory { handle: handle, _phantom: PhantomData }
        }
    }

    /// Creates a reference to the source data for passing into bgfx. When using this constructor
    /// over the `copy` call, no copy will be created. bgfx will read the source memory directly.
    ///
    /// *Note that this is only valid for memory that will live for longer than the bgfx object,
    /// as it's the only way we can guarantee that the memory will still be valid until bgfx has
    /// finished using it.*
    #[inline]
    pub fn reference<T>(_bgfx: &Bgfx, data: &'b [T]) -> Memory<'b> {
        // TODO: The lifetime setup probably isn't 100% complete. Need to figure out how we can
        // guarantee that `data` will outlast `_bgfx`.
        unsafe {
            let handle = bgfx_sys::bgfx_make_ref(data.as_ptr() as *const ::std::os::raw::c_void,
                                                 mem::size_of_val(data) as u32);
            Memory { handle: handle, _phantom: PhantomData }
        }
    }

}

/// Shader program.
///
/// The program holds a vertex shader and a fragment shader.
pub struct Program<'s> {
    handle: bgfx_sys::bgfx_program_handle_t,
    _vsh: Shader<'s>,
    _fsh: Shader<'s>,
}

impl<'s> Program<'s> {

    /// Creates a new program from a vertex shader and a fragment shader. Ownerships of the shaders
    /// are moved to the program.
    #[inline]
    pub fn new(vsh: Shader<'s>, fsh: Shader<'s>) -> Program<'s> {
        unsafe {
            let handle = bgfx_sys::bgfx_create_program(vsh.handle, fsh.handle, false);
            Program { handle: handle, _vsh: vsh, _fsh: fsh }
        }
    }

}

impl<'s> Drop for Program<'s> {

    #[inline]
    fn drop(&mut self) {
        unsafe { bgfx_sys::bgfx_destroy_program(self.handle) }
    }

}

/// Shader.
pub struct Shader<'m> {
    handle: bgfx_sys::bgfx_shader_handle_t,
    _phantom: PhantomData<&'m ()>,
}

impl<'m> Shader<'m> {

    /// Creates a new shader from bgfx-managed memory.
    #[inline]
    pub fn new(data: Memory<'m>) -> Shader<'m> {
        unsafe {
            let handle = bgfx_sys::bgfx_create_shader(data.handle);
            Shader { handle: handle, _phantom: PhantomData }
        }
    }

}

impl<'m> Drop for Shader<'m> {

    #[inline]
    fn drop(&mut self) {
        unsafe { bgfx_sys::bgfx_destroy_shader(self.handle) }
    }

}

/// Vertex index buffer.
pub struct IndexBuffer<'m> {
    handle: bgfx_sys::bgfx_index_buffer_handle_t,
    _phantom: PhantomData<&'m ()>,
}

impl<'m> IndexBuffer<'m> {

    /// Creates a new index buffer from bgfx-managed memory.
    #[inline]
    pub fn new(indices: Memory<'m>, flags: BufferFlags) -> IndexBuffer<'m> {
        unsafe {
            let handle = bgfx_sys::bgfx_create_index_buffer(indices.handle, flags.bits());
            IndexBuffer { handle: handle, _phantom: PhantomData }
        }
    }

}

impl<'m> Drop for IndexBuffer<'m> {

    #[inline]
    fn drop(&mut self) {
        unsafe { bgfx_sys::bgfx_destroy_index_buffer(self.handle) }
    }

}

/// Vertex data buffer.
pub struct VertexBuffer<'m> {
    handle: bgfx_sys::bgfx_vertex_buffer_handle_t,
    _phantom: PhantomData<&'m Bgfx>,
}

impl<'m> VertexBuffer<'m> {

    /// Creates a new vertex buffer from bgfx-managed memory.
    #[inline]
    pub fn new<'v>(verts: Memory<'m>,
                   decl: &'v VertexDecl,
                   flags: BufferFlags)
                   -> VertexBuffer<'m> {
        unsafe {
            let handle = bgfx_sys::bgfx_create_vertex_buffer(verts.handle,
                                                             &decl.decl,
                                                             flags.bits());
            VertexBuffer { handle: handle, _phantom: PhantomData }
        }
    }

}

impl<'m> Drop for VertexBuffer<'m> {

    #[inline]
    fn drop(&mut self) {
        unsafe { bgfx_sys::bgfx_destroy_vertex_buffer(self.handle) }
    }

}

pub struct InstanceDataBuffer<T: 'static>
{
    pub data: &'static mut [T],
    pidb: *const bgfx_sys::bgfx_instance_data_buffer_t
}

impl<T> Drop for InstanceDataBuffer<T> {

    #[inline]
    fn drop(&mut self) {
        panic!("Allocated instance data buffer MUST be given back to bgfx or leaks will happen");
    }

}

/// Describes the structure of a vertex.
pub struct VertexDecl {
    decl: bgfx_sys::bgfx_vertex_decl_t,
}

impl VertexDecl {

    /// Creates a new vertex declaration using a [`VertexDeclBuilder`].
    ///
    /// # Example
    ///
    /// ```
    /// let decl = bgfx::VertexDecl::new(None)
    ///                .add(bgfx::Attrib::Position, 3, bgfx::AttribType::Float)
    ///                .add(bgfx::Attrib::Color0, 4, bgfx::AttribType::Uint8(true))
    ///                .end();
    /// ```
    ///
    /// [`VertexDeclBuilder`]: struct.VertexDeclBuilder.html
    #[inline]
    pub fn new(renderer: Option<RendererType>) -> VertexDeclBuilder {
        let renderer = renderer.unwrap_or(RendererType::Null) as bgfx_sys::bgfx_renderer_type_t;

        unsafe {
            let mut descr = VertexDeclBuilder { decl: mem::uninitialized() };
            bgfx_sys::bgfx_vertex_decl_begin(&mut descr.decl, renderer);
            descr
        }
    }

}

/// Builder for `VertexDecl` instances.
pub struct VertexDeclBuilder {
    decl: bgfx_sys::bgfx_vertex_decl_t,
}

impl VertexDeclBuilder {

    /// Adds a field to the structure descriptor. See [`VertexDecl::new`] for an example.
    ///
    /// [`VertexDecl::new`]: struct.VertexDecl.html#method.new
    pub fn add(&mut self, attrib: Attrib, count: u8, kind: AttribType) -> &mut Self {
        let mut normalized = false;
        let mut as_int = false;

        let kind = match kind {
            AttribType::Uint8(n) => {
                normalized = n;
                bgfx_sys::BGFX_ATTRIB_TYPE_UINT8
            }
            AttribType::Int8(n) => {
                normalized = n;
                as_int = true;
                bgfx_sys::BGFX_ATTRIB_TYPE_UINT8
            }
            AttribType::Uint10(n) => {
                normalized = n;
                bgfx_sys::BGFX_ATTRIB_TYPE_UINT10
            }
            AttribType::Int10(n) => {
                normalized = n;
                as_int = true;
                bgfx_sys::BGFX_ATTRIB_TYPE_UINT10
            }
            AttribType::Uint16(n) => {
                normalized = n;
                bgfx_sys::BGFX_ATTRIB_TYPE_INT16
            }
            AttribType::Int16(n) => {
                normalized = n;
                as_int = true;
                bgfx_sys::BGFX_ATTRIB_TYPE_INT16
            }
            AttribType::Half => bgfx_sys::BGFX_ATTRIB_TYPE_HALF,
            AttribType::Float => bgfx_sys::BGFX_ATTRIB_TYPE_FLOAT,
        };

        unsafe {
            bgfx_sys::bgfx_vertex_decl_add(&mut self.decl,
                                           attrib as bgfx_sys::bgfx_attrib_t,
                                           count,
                                           kind,
                                           normalized,
                                           as_int);
        }

        self
    }

    /// Finalizes the construction of the [`VertexDecl`].
    ///
    /// [`VertexDecl`]: struct.VertexDecl.html
    #[inline]
    pub fn end(&mut self) -> VertexDecl {
        unsafe {
            bgfx_sys::bgfx_vertex_decl_end(&mut self.decl);
        }

        VertexDecl { decl: self.decl }
    }

    /// Indicates a gap in the vertex structure.
    #[inline]
    pub fn skip(&mut self, bytes: u8) -> &mut Self {
        unsafe {
            bgfx_sys::bgfx_vertex_decl_skip(&mut self.decl, bytes);
        }

        self
    }

}


/// Texture.

#[repr(C)]
#[derive(Default, Debug)]
pub struct TextureInfo { // = bgfx_sys::bgfx_texture_info_t
    pub format: TextureFormat,
    pub storageSize: u32,
    pub width: u16,
    pub height: u16,
    pub depth: u16,
    pub numLayers: u16,
    pub numMips: u8,
    pub bitsPerPixel: u8,
    pub cubeMap: bool,
}

pub struct TextureHandle<'m> {
    handle: bgfx_sys::bgfx_texture_handle_t,
    pub info: TextureInfo,
    _phantom: PhantomData<&'m Bgfx>,
}

impl<'m> TextureHandle<'m> {

    /// Creates a new texture from bgfx-managed memory.
    #[inline]
    pub fn new<'v>(buf: Memory<'m>,
                   flags: TextureFlags,
                   skip: u8)
                   -> Self {
        unsafe {
            let mut info: TextureInfo = Default::default();
            let handle = bgfx_sys::bgfx_create_texture(buf.handle,
                                                       flags.bits(),
                                                       skip,
                                                       std::mem::transmute(&mut info));
            Self { handle: handle, info: info, _phantom: PhantomData }
        }
    }

}

impl<'m> Drop for TextureHandle<'m> {

    #[inline]
    fn drop(&mut self) {
        unsafe { bgfx_sys::bgfx_destroy_texture(self.handle) }
    }

}

/// Uniform.
pub struct UniformHandle<'m> {
    handle: bgfx_sys::bgfx_uniform_handle_t,
    _phantom: PhantomData<&'m Bgfx>,
}

impl<'m> UniformHandle<'m> {

    /// Creates a new uniform from bgfx-managed memory.
    #[inline]
    pub fn new<'v>(name: &str,
                   _type: UniformType,
                   num: u16)
                   -> Self {
        unsafe {
            let s = std::ffi::CString::new(name).unwrap();
            let handle = bgfx_sys::bgfx_create_uniform(s.as_ptr(),
                                                       _type as bgfx_sys::bgfx_uniform_type_t,
                                                       num);
            Self { handle: handle, _phantom: PhantomData }
        }
    }

}

impl<'m> Drop for UniformHandle<'m> {

    #[inline]
    fn drop(&mut self) {
        unsafe { bgfx_sys::bgfx_destroy_uniform(self.handle) }
    }

}

// Caps

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct CapsGpu {
    vendorId : VendorId,
    deviceId : u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct CapsLimits { // = bgfx_sys::bgfx_caps_limits
    pub maxDrawCalls: u32,
    pub maxBlits: u32,
    pub maxTextureSize: u32,
    pub maxViews: u32,
    pub maxFrameBuffers: u32,
    pub maxFBAttachments: u32,
    pub maxPrograms: u32,
    pub maxShaders: u32,
    pub maxTextures: u32,
    pub maxTextureSamplers: u32,
    pub maxVertexDecls: u32,
    pub maxVertexStreams: u32,
    pub maxIndexBuffers: u32,
    pub maxVertexBuffers: u32,
    pub maxDynamicIndexBuffers: u32,
    pub maxDynamicVertexBuffers: u32,
    pub maxUniforms: u32,
    pub maxOcclusionQueries: u32,
}

#[repr(C)]
pub struct Caps { // = bgfx_sys::bgfx_caps;
    pub rendererType: RendererType,
    pub supported: CapsFlags,
    pub vendorId: VendorId,
    pub deviceId: u16,
    pub homogeneousDepth: bool,
    pub originBottomLeft: bool,
    pub numGPUs: u8,
    pub gpu: [CapsGpu; 4usize],
    pub limits: CapsLimits,
    pub formats: [TextureCapsFlags; 76usize],
}

impl std::default::Default for Caps {
    fn default() -> Self {
        Self {
            rendererType: RendererType::Default,
            supported: Default::default(),
            vendorId: Default::default(),
            deviceId: 0,
            homogeneousDepth: false,
            originBottomLeft: false,
            numGPUs: 0,
            gpu: [Default::default(); 4usize],
            limits: Default::default(),
            formats: [Default::default(); 76usize],
        }
    }
}

struct FormatsDebugHelper<'a> {
    data: &'a [TextureCapsFlags]
}
impl<'a> fmt::Debug for FormatsDebugHelper<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {} ]", self.data.iter().enumerate().fold(String::new(), |acc, (i,&v)| {
            format!("{}{}0x{:x}", acc, if i == 0 {""} else {", "}, v.bits())
        }))
    }
}

impl fmt::Debug for Caps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Caps").
            field("rendererType", &self.rendererType).
            field("supported", &self.supported).
            field("vendorId", &self.vendorId).
            field("deviceId", &self.deviceId).
            field("homogeneousDepth", &self.homogeneousDepth).
            field("originBottomLeft", &self.originBottomLeft).
            field("numGPUs", &self.numGPUs).
            field("gpu", &self.gpu). // Seems to sometimes contain odd info
            field("limits", &self.limits).
            field("formats", &FormatsDebugHelper{data:&self.formats}).
            finish()
    }
}

/// Acts as the library wrapper for bgfx. Any calls intended to be run on the main thread are
/// exposed as functions on this object.
///
/// It is created through a call to [`bgfx::init`], and will shut down bgfx when dropped.
///
/// [`bgfx::init`]: fn.init.html
pub struct Bgfx {
    // This dummy field only exists so this type can't be publicly instantiated.
    _dummy: u32,
}

impl Bgfx {

    #[inline]
    fn new() -> Bgfx {
        Bgfx { _dummy: 0 }
    }

    /// Gets the caps Bgfx is supporting
    pub fn caps(&self) -> Caps {
        let mut caps: Caps = Default::default();

        unsafe { ::std::ptr::copy(std::mem::transmute::<*const bgfx_sys::bgfx_caps_t, *const Caps>(bgfx_sys::bgfx_get_caps()), &mut caps, 1) }
        caps
    }

    /// Clears the debug text overlay.
    #[inline]
    pub fn dbg_text_clear(&self, attr: Option<u8>, small: Option<bool>) {
        let small = small.unwrap_or(false);
        let attr = attr.unwrap_or(0);

        unsafe { bgfx_sys::bgfx_dbg_text_clear(attr, small) }
    }

    /// Draws an image to the debug text overlay.
    #[inline]
    pub fn dbg_text_image(&self,
                          x: u16,
                          y: u16,
                          width: u16,
                          height: u16,
                          data: &[u8],
                          pitch: u16) {
        unsafe {
            bgfx_sys::bgfx_dbg_text_image(x,
                                          y,
                                          width,
                                          height,
                                          data.as_ptr() as *const ::std::os::raw::c_void,
                                          pitch)
        }
    }

    /// Displays text in the debug text overlay.
    #[inline]
    pub fn dbg_text_print(&self, x: u16, y: u16, attr: u8, text: &str) {
        let text = ffi::CString::new(text).unwrap();
        unsafe { bgfx_sys::bgfx_dbg_text_printf(x, y, attr, text.as_ptr()) }
    }

    /// Finish the frame, syncing up with the render thread. Returns an incrementing frame counter.
    #[inline]
    pub fn frame(&self) -> u32 {
        unsafe { bgfx_sys::bgfx_frame(false) }
    }

    /// Finish the frame, syncing up with the render thread. Returns an incrementing frame counter.
    #[inline]
    pub fn frame_capture(&self) -> u32 {
        unsafe { bgfx_sys::bgfx_frame(true) }
    }

    /// Gets the type of the renderer in use.
    #[inline]
    pub fn get_renderer_type(&self) -> RendererType {
        unsafe { RendererType::from_i32(bgfx_sys::bgfx_get_renderer_type()).unwrap() }
    }

    /// Resets the graphics device to the given size, with the given flags.
    #[inline]
    pub fn reset(&self, width: u16, height: u16, reset: ResetFlags) {
        unsafe { bgfx_sys::bgfx_reset(width as u32, height as u32, reset.bits()) }
    }

    /// Sets the debug flags to use.
    #[inline]
    pub fn set_debug(&self, debug: DebugFlags) {
        unsafe { bgfx_sys::bgfx_set_debug(debug.bits()) }
    }

    pub fn alloc_instance_data_buffer<T>(&self, num: usize) -> Option<InstanceDataBuffer<T>> {
        let stride = mem::size_of::<T>();
        unsafe {
            let pidb: *const bgfx_sys::bgfx_instance_data_buffer_t =
                bgfx_sys::bgfx_alloc_instance_data_buffer(num as u32, stride as u16);
            if pidb.is_null() {
                return None;
            }
            let data: *mut T = mem::transmute((*pidb).data);
            let idb = InstanceDataBuffer::<T> {
                data: std::slice::from_raw_parts_mut::<T>(data, num),
                pidb: pidb,
            };
            Some(idb)
        }
    }

    /// Sets the index buffer to use for rendering.
    #[inline]
    pub fn set_index_buffer(&self, ibh: &IndexBuffer) {
        // TODO: How to solve lifetimes...
        unsafe { bgfx_sys::bgfx_set_index_buffer(ibh.handle, 0, std::u32::MAX) }
    }

    /// Sets the render state.
    #[inline]
    pub fn set_state(&self, state: StateFlags, rgba: Option<u32>) {
        unsafe { bgfx_sys::bgfx_set_state(state.bits(), rgba.unwrap_or(0)) }
    }

    /// Sets the model transform for rendering. If not called before submitting a draw, an identity
    /// matrix will be used.
    #[inline]
    pub fn set_transform(&self, mtx: &[f32; 16]) {
        unsafe {
            bgfx_sys::bgfx_set_transform(mtx.as_ptr() as *const ::std::os::raw::c_void, 1);
        }
    }

    /// Sets the vertex buffer to use for rendering.
    #[inline]
    pub fn set_vertex_buffer(&self, vbh: &VertexBuffer) {
        // TODO: How to solve lifetimes...
        unsafe { bgfx_sys::bgfx_set_vertex_buffer(vbh.handle, 0, std::u32::MAX) }
    }

    /// Sets and consumes the allocated instance data buffer
    pub fn set_instance_data_buffer<T>(&self, idb: InstanceDataBuffer<T>) {
        unsafe {
            bgfx_sys::bgfx_set_instance_data_buffer(idb.pidb, std::u32::MAX);
            mem::forget(idb);
        }
    }

    /// Sets a texture to a sampler.
    #[inline]
    pub fn set_texture(&self, stage: u8, uh: &UniformHandle, th: &TextureHandle) {
        // TODO: How to solve lifetimes...
        unsafe { bgfx_sys::bgfx_set_texture(stage, uh.handle, th.handle, u32::max_value()) }
    }

    /// Sets the options to use when clearing the given view.
    #[inline]
    pub fn set_view_clear(&self, id: u8, flags: ClearFlags, rgba: u32, depth: f32, stencil: u8) {
        unsafe { bgfx_sys::bgfx_set_view_clear(id, flags.bits(), rgba, depth, stencil) }
    }

    /// Sets the rectangle to display the given view in.
    #[inline]
    pub fn set_view_rect(&self, id: u8, x: u16, y: u16, width: u16, height: u16) {
        unsafe { bgfx_sys::bgfx_set_view_rect(id, x, y, width, height) }
    }

    /// Sets the view and projection matrices for the given view.
    #[inline]
    pub fn set_view_transform(&self, id: u8, view: &[f32; 16], proj: &[f32; 16]) {
        unsafe {
            bgfx_sys::bgfx_set_view_transform(id,
                                              view.as_ptr() as *const ::std::os::raw::c_void,
                                              proj.as_ptr() as *const ::std::os::raw::c_void)
        }
    }

    /// Submit a primitive for rendering. Returns the number of draw calls used.
    #[inline]
    pub fn submit(&self, view: u8, program: &Program) -> u32 {
        unsafe { bgfx_sys::bgfx_submit(view, program.handle, 0, false) }
    }

    /// Touches a view. ( ͡° ͜ʖ ͡°)
    #[inline]
    pub fn touch(&self, id: u8) {
        unsafe {
            bgfx_sys::bgfx_touch(id);
        }
    }

}

impl Drop for Bgfx {

    #[inline]
    fn drop(&mut self) {
        unsafe { bgfx_sys::bgfx_shutdown() }
    }

}

/// Pump the render thread.
///
/// This should be called repeatedly on the render thread.
#[inline]
pub fn render_frame() -> RenderFrame {
    unsafe { RenderFrame::from_i32(bgfx_sys::bgfx_render_frame()).unwrap() }
}

/// Platform data initializer.
///
/// This should be applied *only once*, before bgfx is used.
///
/// # Example
///
/// ```should_panic
/// // Note: The default value for all of these options is null. If that is what you want, you may
/// // choose not to call said setter.
/// bgfx::PlatformData::new()
///     .context(std::ptr::null_mut())
///     .display(std::ptr::null_mut()) // Must be non-null on unix platforms
///     .window(std::ptr::null_mut()) // Must be non-null
///     .apply()
///     .expect("Could not set platform data");
/// ```
pub struct PlatformData {
    data: bgfx_sys::bgfx_platform_data,
}

impl PlatformData {

    /// Creates an empty PlatformData instance.
    #[inline]
    pub fn new() -> PlatformData {
        PlatformData {
            data: bgfx_sys::bgfx_platform_data {
                ndt: ptr::null_mut(),
                nwh: ptr::null_mut(),
                context: ptr::null_mut(),
                backBuffer: ptr::null_mut(),
                backBufferDS: ptr::null_mut(),
                session: ptr::null_mut(),
            },
        }
    }

    /// Apply the platform configuration.
    pub fn apply(&mut self) -> Result<(), BgfxError> {
        if self.data.ndt == ptr::null_mut() && cfg!(target_os = "linux") {
            Err(BgfxError::InvalidDisplay)
        } else if self.data.nwh == ptr::null_mut() {
            Err(BgfxError::InvalidWindow)
        } else {
            unsafe {
                bgfx_sys::bgfx_set_platform_data(&mut self.data);
            }
            Ok(())
        }
    }

    /// Sets the GL context to use.
    #[inline]
    pub fn context(&mut self, context: *mut ::std::os::raw::c_void) -> &mut Self {
        self.data.context = context;
        self
    }

    /// Sets the X11 display to use on unix systems.
    #[inline]
    pub fn display(&mut self, display: *mut ::std::os::raw::c_void) -> &mut Self {
        self.data.ndt = display;
        self
    }

    /// Sets the handle to the window to use.
    #[inline]
    pub fn window(&mut self, window: *mut ::std::os::raw::c_void) -> &mut Self {
        self.data.nwh = window;
        self
    }

}

/// Initializes bgfx.
///
/// This must be called on the main thread after setting the platform data. See [`PlatformData`].
///
/// [`PlatformData`]: struct.PlatformData.html
pub fn init(renderer: RendererType,
            vendor_id: Option<VendorId>,
            device_id: Option<u16>)
            -> Result<Bgfx, BgfxError> {
    let renderer = renderer as bgfx_sys::bgfx_renderer_type_t;
    let vendor = vendor_id.unwrap_or(Default::default());
    let device = device_id.unwrap_or(0);

    unsafe {
        let success = bgfx_sys::bgfx_init(renderer,
                                          vendor as u16,
                                          device,
                                          ptr::null_mut(),
                                          ptr::null_mut());

        if success { Ok(Bgfx::new()) } else { Err(BgfxError::InitFailed) }
    }
}
