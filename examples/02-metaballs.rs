// Copyright (c) 2015-2016, Johan Sköld.
// Copyright (c) 2017, Javier Arevalo
// License: http://opensource.org/licenses/ISC
//
// Original code: Copyright 2011-2017 Branimir Karadzic. All rights reserved.
// License: https://github.com/bkaradzic/bgfx#license-bsd-2-clause

extern crate bgfx;
extern crate cgmath;
extern crate glutin;
extern crate time;

mod common;

use bgfx::*;
use cgmath::{Decomposed, Deg, Matrix4, Point3, Quaternion, Rad, Transform, Vector3, Euler, InnerSpace, Zero};
use common::*;
use time::PreciseTime;
use std::default::Default;

const VS_METABALLS_OPENGL: &'static [u8] = include_bytes!("assets/02-metaballs/OpenGL/vs_metaballs.bin");
const FS_METABALLS_OPENGL: &'static [u8] = include_bytes!("assets/02-metaballs/OpenGL/fs_metaballs.bin");
const VS_METABALLS_D3D11: &'static [u8] = include_bytes!("assets/02-metaballs/Direct3D11/vs_metaballs.bin");
const FS_METABALLS_D3D11: &'static [u8] = include_bytes!("assets/02-metaballs/Direct3D11/fs_metaballs.bin");

#[repr(packed)]
struct PosNormalColorVertex {
    _x: f32,
    _y: f32,
    _z: f32,
    _nx: f32,
    _ny: f32,
    _nz: f32,
    _abgr: u32,
}

impl PosNormalColorVertex {
    fn build_decl() -> VertexDecl {
        VertexDecl::new(None)
            .add(Attrib::Position, 3, AttribType::Float)
            .add(Attrib::Normal, 3, AttribType::Float)
            .add(Attrib::Color0, 4, AttribType::Uint8(true))
            .end()
    }
}

#[derive(Copy, Clone)]
struct Grid
{
	val: f32,
	normal: Vector3<f32>
}

impl Default for Grid {
	fn default() -> Grid {
        Grid {
            val: 0f32,
            normal: Vector3::zero()
        }
    }
}

// Triangulation tables taken from:
// http://paulbourke.net/geometry/polygonise/

const S_EDGES: [u16; 256] =
[
	0x000, 0x109, 0x203, 0x30a, 0x406, 0x50f, 0x605, 0x70c,
	0x80c, 0x905, 0xa0f, 0xb06, 0xc0a, 0xd03, 0xe09, 0xf00,
	0x190, 0x099, 0x393, 0x29a, 0x596, 0x49f, 0x795, 0x69c,
	0x99c, 0x895, 0xb9f, 0xa96, 0xd9a, 0xc93, 0xf99, 0xe90,
	0x230, 0x339, 0x033, 0x13a, 0x636, 0x73f, 0x435, 0x53c,
	0xa3c, 0xb35, 0x83f, 0x936, 0xe3a, 0xf33, 0xc39, 0xd30,
	0x3a0, 0x2a9, 0x1a3, 0x0aa, 0x7a6, 0x6af, 0x5a5, 0x4ac,
	0xbac, 0xaa5, 0x9af, 0x8a6, 0xfaa, 0xea3, 0xda9, 0xca0,
	0x460, 0x569, 0x663, 0x76a, 0x66 , 0x16f, 0x265, 0x36c,
	0xc6c, 0xd65, 0xe6f, 0xf66, 0x86a, 0x963, 0xa69, 0xb60,
	0x5f0, 0x4f9, 0x7f3, 0x6fa, 0x1f6, 0x0ff, 0x3f5, 0x2fc,
	0xdfc, 0xcf5, 0xfff, 0xef6, 0x9fa, 0x8f3, 0xbf9, 0xaf0,
	0x650, 0x759, 0x453, 0x55a, 0x256, 0x35f, 0x055, 0x15c,
	0xe5c, 0xf55, 0xc5f, 0xd56, 0xa5a, 0xb53, 0x859, 0x950,
	0x7c0, 0x6c9, 0x5c3, 0x4ca, 0x3c6, 0x2cf, 0x1c5, 0x0cc,
	0xfcc, 0xec5, 0xdcf, 0xcc6, 0xbca, 0xac3, 0x9c9, 0x8c0,
	0x8c0, 0x9c9, 0xac3, 0xbca, 0xcc6, 0xdcf, 0xec5, 0xfcc,
	0x0cc, 0x1c5, 0x2cf, 0x3c6, 0x4ca, 0x5c3, 0x6c9, 0x7c0,
	0x950, 0x859, 0xb53, 0xa5a, 0xd56, 0xc5f, 0xf55, 0xe5c,
	0x15c, 0x55 , 0x35f, 0x256, 0x55a, 0x453, 0x759, 0x650,
	0xaf0, 0xbf9, 0x8f3, 0x9fa, 0xef6, 0xfff, 0xcf5, 0xdfc,
	0x2fc, 0x3f5, 0x0ff, 0x1f6, 0x6fa, 0x7f3, 0x4f9, 0x5f0,
	0xb60, 0xa69, 0x963, 0x86a, 0xf66, 0xe6f, 0xd65, 0xc6c,
	0x36c, 0x265, 0x16f, 0x066, 0x76a, 0x663, 0x569, 0x460,
	0xca0, 0xda9, 0xea3, 0xfaa, 0x8a6, 0x9af, 0xaa5, 0xbac,
	0x4ac, 0x5a5, 0x6af, 0x7a6, 0x0aa, 0x1a3, 0x2a9, 0x3a0,
	0xd30, 0xc39, 0xf33, 0xe3a, 0x936, 0x83f, 0xb35, 0xa3c,
	0x53c, 0x435, 0x73f, 0x636, 0x13a, 0x033, 0x339, 0x230,
	0xe90, 0xf99, 0xc93, 0xd9a, 0xa96, 0xb9f, 0x895, 0x99c,
	0x69c, 0x795, 0x49f, 0x596, 0x29a, 0x393, 0x099, 0x190,
	0xf00, 0xe09, 0xd03, 0xc0a, 0xb06, 0xa0f, 0x905, 0x80c,
	0x70c, 0x605, 0x50f, 0x406, 0x30a, 0x203, 0x109, 0x000,
];

const S_INDICES: [[i8; 16]; 256] =
[
	[  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   0,  8,  3, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   0,  1,  9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   1,  8,  3,  9,  8,  1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   1,  2, 10, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   0,  8,  3,  1,  2, 10, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   9,  2, 10,  0,  2,  9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   2,  8,  3,  2, 10,  8, 10,  9,  8, -1, -1, -1, -1, -1, -1, -1 ],
	[   3, 11,  2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   0, 11,  2,  8, 11,  0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   1,  9,  0,  2,  3, 11, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   1, 11,  2,  1,  9, 11,  9,  8, 11, -1, -1, -1, -1, -1, -1, -1 ],
	[   3, 10,  1, 11, 10,  3, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   0, 10,  1,  0,  8, 10,  8, 11, 10, -1, -1, -1, -1, -1, -1, -1 ],
	[   3,  9,  0,  3, 11,  9, 11, 10,  9, -1, -1, -1, -1, -1, -1, -1 ],
	[   9,  8, 10, 10,  8, 11, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   4,  7,  8, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   4,  3,  0,  7,  3,  4, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   0,  1,  9,  8,  4,  7, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   4,  1,  9,  4,  7,  1,  7,  3,  1, -1, -1, -1, -1, -1, -1, -1 ],
	[   1,  2, 10,  8,  4,  7, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   3,  4,  7,  3,  0,  4,  1,  2, 10, -1, -1, -1, -1, -1, -1, -1 ],
	[   9,  2, 10,  9,  0,  2,  8,  4,  7, -1, -1, -1, -1, -1, -1, -1 ],
	[   2, 10,  9,  2,  9,  7,  2,  7,  3,  7,  9,  4, -1, -1, -1, -1 ],
	[   8,  4,  7,  3, 11,  2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[  11,  4,  7, 11,  2,  4,  2,  0,  4, -1, -1, -1, -1, -1, -1, -1 ],
	[   9,  0,  1,  8,  4,  7,  2,  3, 11, -1, -1, -1, -1, -1, -1, -1 ],
	[   4,  7, 11,  9,  4, 11,  9, 11,  2,  9,  2,  1, -1, -1, -1, -1 ],
	[   3, 10,  1,  3, 11, 10,  7,  8,  4, -1, -1, -1, -1, -1, -1, -1 ],
	[   1, 11, 10,  1,  4, 11,  1,  0,  4,  7, 11,  4, -1, -1, -1, -1 ],
	[   4,  7,  8,  9,  0, 11,  9, 11, 10, 11,  0,  3, -1, -1, -1, -1 ],
	[   4,  7, 11,  4, 11,  9,  9, 11, 10, -1, -1, -1, -1, -1, -1, -1 ],
	[   9,  5,  4, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   9,  5,  4,  0,  8,  3, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   0,  5,  4,  1,  5,  0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   8,  5,  4,  8,  3,  5,  3,  1,  5, -1, -1, -1, -1, -1, -1, -1 ],
	[   1,  2, 10,  9,  5,  4, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   3,  0,  8,  1,  2, 10,  4,  9,  5, -1, -1, -1, -1, -1, -1, -1 ],
	[   5,  2, 10,  5,  4,  2,  4,  0,  2, -1, -1, -1, -1, -1, -1, -1 ],
	[   2, 10,  5,  3,  2,  5,  3,  5,  4,  3,  4,  8, -1, -1, -1, -1 ],
	[   9,  5,  4,  2,  3, 11, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   0, 11,  2,  0,  8, 11,  4,  9,  5, -1, -1, -1, -1, -1, -1, -1 ],
	[   0,  5,  4,  0,  1,  5,  2,  3, 11, -1, -1, -1, -1, -1, -1, -1 ],
	[   2,  1,  5,  2,  5,  8,  2,  8, 11,  4,  8,  5, -1, -1, -1, -1 ],
	[  10,  3, 11, 10,  1,  3,  9,  5,  4, -1, -1, -1, -1, -1, -1, -1 ],
	[   4,  9,  5,  0,  8,  1,  8, 10,  1,  8, 11, 10, -1, -1, -1, -1 ],
	[   5,  4,  0,  5,  0, 11,  5, 11, 10, 11,  0,  3, -1, -1, -1, -1 ],
	[   5,  4,  8,  5,  8, 10, 10,  8, 11, -1, -1, -1, -1, -1, -1, -1 ],
	[   9,  7,  8,  5,  7,  9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   9,  3,  0,  9,  5,  3,  5,  7,  3, -1, -1, -1, -1, -1, -1, -1 ],
	[   0,  7,  8,  0,  1,  7,  1,  5,  7, -1, -1, -1, -1, -1, -1, -1 ],
	[   1,  5,  3,  3,  5,  7, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   9,  7,  8,  9,  5,  7, 10,  1,  2, -1, -1, -1, -1, -1, -1, -1 ],
	[  10,  1,  2,  9,  5,  0,  5,  3,  0,  5,  7,  3, -1, -1, -1, -1 ],
	[   8,  0,  2,  8,  2,  5,  8,  5,  7, 10,  5,  2, -1, -1, -1, -1 ],
	[   2, 10,  5,  2,  5,  3,  3,  5,  7, -1, -1, -1, -1, -1, -1, -1 ],
	[   7,  9,  5,  7,  8,  9,  3, 11,  2, -1, -1, -1, -1, -1, -1, -1 ],
	[   9,  5,  7,  9,  7,  2,  9,  2,  0,  2,  7, 11, -1, -1, -1, -1 ],
	[   2,  3, 11,  0,  1,  8,  1,  7,  8,  1,  5,  7, -1, -1, -1, -1 ],
	[  11,  2,  1, 11,  1,  7,  7,  1,  5, -1, -1, -1, -1, -1, -1, -1 ],
	[   9,  5,  8,  8,  5,  7, 10,  1,  3, 10,  3, 11, -1, -1, -1, -1 ],
	[   5,  7,  0,  5,  0,  9,  7, 11,  0,  1,  0, 10, 11, 10,  0, -1 ],
	[  11, 10,  0, 11,  0,  3, 10,  5,  0,  8,  0,  7,  5,  7,  0, -1 ],
	[  11, 10,  5,  7, 11,  5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[  10,  6,  5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   0,  8,  3,  5, 10,  6, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   9,  0,  1,  5, 10,  6, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   1,  8,  3,  1,  9,  8,  5, 10,  6, -1, -1, -1, -1, -1, -1, -1 ],
	[   1,  6,  5,  2,  6,  1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   1,  6,  5,  1,  2,  6,  3,  0,  8, -1, -1, -1, -1, -1, -1, -1 ],
	[   9,  6,  5,  9,  0,  6,  0,  2,  6, -1, -1, -1, -1, -1, -1, -1 ],
	[   5,  9,  8,  5,  8,  2,  5,  2,  6,  3,  2,  8, -1, -1, -1, -1 ],
	[   2,  3, 11, 10,  6,  5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[  11,  0,  8, 11,  2,  0, 10,  6,  5, -1, -1, -1, -1, -1, -1, -1 ],
	[   0,  1,  9,  2,  3, 11,  5, 10,  6, -1, -1, -1, -1, -1, -1, -1 ],
	[   5, 10,  6,  1,  9,  2,  9, 11,  2,  9,  8, 11, -1, -1, -1, -1 ],
	[   6,  3, 11,  6,  5,  3,  5,  1,  3, -1, -1, -1, -1, -1, -1, -1 ],
	[   0,  8, 11,  0, 11,  5,  0,  5,  1,  5, 11,  6, -1, -1, -1, -1 ],
	[   3, 11,  6,  0,  3,  6,  0,  6,  5,  0,  5,  9, -1, -1, -1, -1 ],
	[   6,  5,  9,  6,  9, 11, 11,  9,  8, -1, -1, -1, -1, -1, -1, -1 ],
	[   5, 10,  6,  4,  7,  8, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   4,  3,  0,  4,  7,  3,  6,  5, 10, -1, -1, -1, -1, -1, -1, -1 ],
	[   1,  9,  0,  5, 10,  6,  8,  4,  7, -1, -1, -1, -1, -1, -1, -1 ],
	[  10,  6,  5,  1,  9,  7,  1,  7,  3,  7,  9,  4, -1, -1, -1, -1 ],
	[   6,  1,  2,  6,  5,  1,  4,  7,  8, -1, -1, -1, -1, -1, -1, -1 ],
	[   1,  2,  5,  5,  2,  6,  3,  0,  4,  3,  4,  7, -1, -1, -1, -1 ],
	[   8,  4,  7,  9,  0,  5,  0,  6,  5,  0,  2,  6, -1, -1, -1, -1 ],
	[   7,  3,  9,  7,  9,  4,  3,  2,  9,  5,  9,  6,  2,  6,  9, -1 ],
	[   3, 11,  2,  7,  8,  4, 10,  6,  5, -1, -1, -1, -1, -1, -1, -1 ],
	[   5, 10,  6,  4,  7,  2,  4,  2,  0,  2,  7, 11, -1, -1, -1, -1 ],
	[   0,  1,  9,  4,  7,  8,  2,  3, 11,  5, 10,  6, -1, -1, -1, -1 ],
	[   9,  2,  1,  9, 11,  2,  9,  4, 11,  7, 11,  4,  5, 10,  6, -1 ],
	[   8,  4,  7,  3, 11,  5,  3,  5,  1,  5, 11,  6, -1, -1, -1, -1 ],
	[   5,  1, 11,  5, 11,  6,  1,  0, 11,  7, 11,  4,  0,  4, 11, -1 ],
	[   0,  5,  9,  0,  6,  5,  0,  3,  6, 11,  6,  3,  8,  4,  7, -1 ],
	[   6,  5,  9,  6,  9, 11,  4,  7,  9,  7, 11,  9, -1, -1, -1, -1 ],
	[  10,  4,  9,  6,  4, 10, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   4, 10,  6,  4,  9, 10,  0,  8,  3, -1, -1, -1, -1, -1, -1, -1 ],
	[  10,  0,  1, 10,  6,  0,  6,  4,  0, -1, -1, -1, -1, -1, -1, -1 ],
	[   8,  3,  1,  8,  1,  6,  8,  6,  4,  6,  1, 10, -1, -1, -1, -1 ],
	[   1,  4,  9,  1,  2,  4,  2,  6,  4, -1, -1, -1, -1, -1, -1, -1 ],
	[   3,  0,  8,  1,  2,  9,  2,  4,  9,  2,  6,  4, -1, -1, -1, -1 ],
	[   0,  2,  4,  4,  2,  6, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   8,  3,  2,  8,  2,  4,  4,  2,  6, -1, -1, -1, -1, -1, -1, -1 ],
	[  10,  4,  9, 10,  6,  4, 11,  2,  3, -1, -1, -1, -1, -1, -1, -1 ],
	[   0,  8,  2,  2,  8, 11,  4,  9, 10,  4, 10,  6, -1, -1, -1, -1 ],
	[   3, 11,  2,  0,  1,  6,  0,  6,  4,  6,  1, 10, -1, -1, -1, -1 ],
	[   6,  4,  1,  6,  1, 10,  4,  8,  1,  2,  1, 11,  8, 11,  1, -1 ],
	[   9,  6,  4,  9,  3,  6,  9,  1,  3, 11,  6,  3, -1, -1, -1, -1 ],
	[   8, 11,  1,  8,  1,  0, 11,  6,  1,  9,  1,  4,  6,  4,  1, -1 ],
	[   3, 11,  6,  3,  6,  0,  0,  6,  4, -1, -1, -1, -1, -1, -1, -1 ],
	[   6,  4,  8, 11,  6,  8, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   7, 10,  6,  7,  8, 10,  8,  9, 10, -1, -1, -1, -1, -1, -1, -1 ],
	[   0,  7,  3,  0, 10,  7,  0,  9, 10,  6,  7, 10, -1, -1, -1, -1 ],
	[  10,  6,  7,  1, 10,  7,  1,  7,  8,  1,  8,  0, -1, -1, -1, -1 ],
	[  10,  6,  7, 10,  7,  1,  1,  7,  3, -1, -1, -1, -1, -1, -1, -1 ],
	[   1,  2,  6,  1,  6,  8,  1,  8,  9,  8,  6,  7, -1, -1, -1, -1 ],
	[   2,  6,  9,  2,  9,  1,  6,  7,  9,  0,  9,  3,  7,  3,  9, -1 ],
	[   7,  8,  0,  7,  0,  6,  6,  0,  2, -1, -1, -1, -1, -1, -1, -1 ],
	[   7,  3,  2,  6,  7,  2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   2,  3, 11, 10,  6,  8, 10,  8,  9,  8,  6,  7, -1, -1, -1, -1 ],
	[   2,  0,  7,  2,  7, 11,  0,  9,  7,  6,  7, 10,  9, 10,  7, -1 ],
	[   1,  8,  0,  1,  7,  8,  1, 10,  7,  6,  7, 10,  2,  3, 11, -1 ],
	[  11,  2,  1, 11,  1,  7, 10,  6,  1,  6,  7,  1, -1, -1, -1, -1 ],
	[   8,  9,  6,  8,  6,  7,  9,  1,  6, 11,  6,  3,  1,  3,  6, -1 ],
	[   0,  9,  1, 11,  6,  7, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   7,  8,  0,  7,  0,  6,  3, 11,  0, 11,  6,  0, -1, -1, -1, -1 ],
	[   7, 11,  6, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   7,  6, 11, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   3,  0,  8, 11,  7,  6, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   0,  1,  9, 11,  7,  6, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   8,  1,  9,  8,  3,  1, 11,  7,  6, -1, -1, -1, -1, -1, -1, -1 ],
	[  10,  1,  2,  6, 11,  7, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   1,  2, 10,  3,  0,  8,  6, 11,  7, -1, -1, -1, -1, -1, -1, -1 ],
	[   2,  9,  0,  2, 10,  9,  6, 11,  7, -1, -1, -1, -1, -1, -1, -1 ],
	[   6, 11,  7,  2, 10,  3, 10,  8,  3, 10,  9,  8, -1, -1, -1, -1 ],
	[   7,  2,  3,  6,  2,  7, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   7,  0,  8,  7,  6,  0,  6,  2,  0, -1, -1, -1, -1, -1, -1, -1 ],
	[   2,  7,  6,  2,  3,  7,  0,  1,  9, -1, -1, -1, -1, -1, -1, -1 ],
	[   1,  6,  2,  1,  8,  6,  1,  9,  8,  8,  7,  6, -1, -1, -1, -1 ],
	[  10,  7,  6, 10,  1,  7,  1,  3,  7, -1, -1, -1, -1, -1, -1, -1 ],
	[  10,  7,  6,  1,  7, 10,  1,  8,  7,  1,  0,  8, -1, -1, -1, -1 ],
	[   0,  3,  7,  0,  7, 10,  0, 10,  9,  6, 10,  7, -1, -1, -1, -1 ],
	[   7,  6, 10,  7, 10,  8,  8, 10,  9, -1, -1, -1, -1, -1, -1, -1 ],
	[   6,  8,  4, 11,  8,  6, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   3,  6, 11,  3,  0,  6,  0,  4,  6, -1, -1, -1, -1, -1, -1, -1 ],
	[   8,  6, 11,  8,  4,  6,  9,  0,  1, -1, -1, -1, -1, -1, -1, -1 ],
	[   9,  4,  6,  9,  6,  3,  9,  3,  1, 11,  3,  6, -1, -1, -1, -1 ],
	[   6,  8,  4,  6, 11,  8,  2, 10,  1, -1, -1, -1, -1, -1, -1, -1 ],
	[   1,  2, 10,  3,  0, 11,  0,  6, 11,  0,  4,  6, -1, -1, -1, -1 ],
	[   4, 11,  8,  4,  6, 11,  0,  2,  9,  2, 10,  9, -1, -1, -1, -1 ],
	[  10,  9,  3, 10,  3,  2,  9,  4,  3, 11,  3,  6,  4,  6,  3, -1 ],
	[   8,  2,  3,  8,  4,  2,  4,  6,  2, -1, -1, -1, -1, -1, -1, -1 ],
	[   0,  4,  2,  4,  6,  2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   1,  9,  0,  2,  3,  4,  2,  4,  6,  4,  3,  8, -1, -1, -1, -1 ],
	[   1,  9,  4,  1,  4,  2,  2,  4,  6, -1, -1, -1, -1, -1, -1, -1 ],
	[   8,  1,  3,  8,  6,  1,  8,  4,  6,  6, 10,  1, -1, -1, -1, -1 ],
	[  10,  1,  0, 10,  0,  6,  6,  0,  4, -1, -1, -1, -1, -1, -1, -1 ],
	[   4,  6,  3,  4,  3,  8,  6, 10,  3,  0,  3,  9, 10,  9,  3, -1 ],
	[  10,  9,  4,  6, 10,  4, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   4,  9,  5,  7,  6, 11, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   0,  8,  3,  4,  9,  5, 11,  7,  6, -1, -1, -1, -1, -1, -1, -1 ],
	[   5,  0,  1,  5,  4,  0,  7,  6, 11, -1, -1, -1, -1, -1, -1, -1 ],
	[  11,  7,  6,  8,  3,  4,  3,  5,  4,  3,  1,  5, -1, -1, -1, -1 ],
	[   9,  5,  4, 10,  1,  2,  7,  6, 11, -1, -1, -1, -1, -1, -1, -1 ],
	[   6, 11,  7,  1,  2, 10,  0,  8,  3,  4,  9,  5, -1, -1, -1, -1 ],
	[   7,  6, 11,  5,  4, 10,  4,  2, 10,  4,  0,  2, -1, -1, -1, -1 ],
	[   3,  4,  8,  3,  5,  4,  3,  2,  5, 10,  5,  2, 11,  7,  6, -1 ],
	[   7,  2,  3,  7,  6,  2,  5,  4,  9, -1, -1, -1, -1, -1, -1, -1 ],
	[   9,  5,  4,  0,  8,  6,  0,  6,  2,  6,  8,  7, -1, -1, -1, -1 ],
	[   3,  6,  2,  3,  7,  6,  1,  5,  0,  5,  4,  0, -1, -1, -1, -1 ],
	[   6,  2,  8,  6,  8,  7,  2,  1,  8,  4,  8,  5,  1,  5,  8, -1 ],
	[   9,  5,  4, 10,  1,  6,  1,  7,  6,  1,  3,  7, -1, -1, -1, -1 ],
	[   1,  6, 10,  1,  7,  6,  1,  0,  7,  8,  7,  0,  9,  5,  4, -1 ],
	[   4,  0, 10,  4, 10,  5,  0,  3, 10,  6, 10,  7,  3,  7, 10, -1 ],
	[   7,  6, 10,  7, 10,  8,  5,  4, 10,  4,  8, 10, -1, -1, -1, -1 ],
	[   6,  9,  5,  6, 11,  9, 11,  8,  9, -1, -1, -1, -1, -1, -1, -1 ],
	[   3,  6, 11,  0,  6,  3,  0,  5,  6,  0,  9,  5, -1, -1, -1, -1 ],
	[   0, 11,  8,  0,  5, 11,  0,  1,  5,  5,  6, 11, -1, -1, -1, -1 ],
	[   6, 11,  3,  6,  3,  5,  5,  3,  1, -1, -1, -1, -1, -1, -1, -1 ],
	[   1,  2, 10,  9,  5, 11,  9, 11,  8, 11,  5,  6, -1, -1, -1, -1 ],
	[   0, 11,  3,  0,  6, 11,  0,  9,  6,  5,  6,  9,  1,  2, 10, -1 ],
	[  11,  8,  5, 11,  5,  6,  8,  0,  5, 10,  5,  2,  0,  2,  5, -1 ],
	[   6, 11,  3,  6,  3,  5,  2, 10,  3, 10,  5,  3, -1, -1, -1, -1 ],
	[   5,  8,  9,  5,  2,  8,  5,  6,  2,  3,  8,  2, -1, -1, -1, -1 ],
	[   9,  5,  6,  9,  6,  0,  0,  6,  2, -1, -1, -1, -1, -1, -1, -1 ],
	[   1,  5,  8,  1,  8,  0,  5,  6,  8,  3,  8,  2,  6,  2,  8, -1 ],
	[   1,  5,  6,  2,  1,  6, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   1,  3,  6,  1,  6, 10,  3,  8,  6,  5,  6,  9,  8,  9,  6, -1 ],
	[  10,  1,  0, 10,  0,  6,  9,  5,  0,  5,  6,  0, -1, -1, -1, -1 ],
	[   0,  3,  8,  5,  6, 10, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[  10,  5,  6, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[  11,  5, 10,  7,  5, 11, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[  11,  5, 10, 11,  7,  5,  8,  3,  0, -1, -1, -1, -1, -1, -1, -1 ],
	[   5, 11,  7,  5, 10, 11,  1,  9,  0, -1, -1, -1, -1, -1, -1, -1 ],
	[  10,  7,  5, 10, 11,  7,  9,  8,  1,  8,  3,  1, -1, -1, -1, -1 ],
	[  11,  1,  2, 11,  7,  1,  7,  5,  1, -1, -1, -1, -1, -1, -1, -1 ],
	[   0,  8,  3,  1,  2,  7,  1,  7,  5,  7,  2, 11, -1, -1, -1, -1 ],
	[   9,  7,  5,  9,  2,  7,  9,  0,  2,  2, 11,  7, -1, -1, -1, -1 ],
	[   7,  5,  2,  7,  2, 11,  5,  9,  2,  3,  2,  8,  9,  8,  2, -1 ],
	[   2,  5, 10,  2,  3,  5,  3,  7,  5, -1, -1, -1, -1, -1, -1, -1 ],
	[   8,  2,  0,  8,  5,  2,  8,  7,  5, 10,  2,  5, -1, -1, -1, -1 ],
	[   9,  0,  1,  5, 10,  3,  5,  3,  7,  3, 10,  2, -1, -1, -1, -1 ],
	[   9,  8,  2,  9,  2,  1,  8,  7,  2, 10,  2,  5,  7,  5,  2, -1 ],
	[   1,  3,  5,  3,  7,  5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   0,  8,  7,  0,  7,  1,  1,  7,  5, -1, -1, -1, -1, -1, -1, -1 ],
	[   9,  0,  3,  9,  3,  5,  5,  3,  7, -1, -1, -1, -1, -1, -1, -1 ],
	[   9,  8,  7,  5,  9,  7, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   5,  8,  4,  5, 10,  8, 10, 11,  8, -1, -1, -1, -1, -1, -1, -1 ],
	[   5,  0,  4,  5, 11,  0,  5, 10, 11, 11,  3,  0, -1, -1, -1, -1 ],
	[   0,  1,  9,  8,  4, 10,  8, 10, 11, 10,  4,  5, -1, -1, -1, -1 ],
	[  10, 11,  4, 10,  4,  5, 11,  3,  4,  9,  4,  1,  3,  1,  4, -1 ],
	[   2,  5,  1,  2,  8,  5,  2, 11,  8,  4,  5,  8, -1, -1, -1, -1 ],
	[   0,  4, 11,  0, 11,  3,  4,  5, 11,  2, 11,  1,  5,  1, 11, -1 ],
	[   0,  2,  5,  0,  5,  9,  2, 11,  5,  4,  5,  8, 11,  8,  5, -1 ],
	[   9,  4,  5,  2, 11,  3, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   2,  5, 10,  3,  5,  2,  3,  4,  5,  3,  8,  4, -1, -1, -1, -1 ],
	[   5, 10,  2,  5,  2,  4,  4,  2,  0, -1, -1, -1, -1, -1, -1, -1 ],
	[   3, 10,  2,  3,  5, 10,  3,  8,  5,  4,  5,  8,  0,  1,  9, -1 ],
	[   5, 10,  2,  5,  2,  4,  1,  9,  2,  9,  4,  2, -1, -1, -1, -1 ],
	[   8,  4,  5,  8,  5,  3,  3,  5,  1, -1, -1, -1, -1, -1, -1, -1 ],
	[   0,  4,  5,  1,  0,  5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   8,  4,  5,  8,  5,  3,  9,  0,  5,  0,  3,  5, -1, -1, -1, -1 ],
	[   9,  4,  5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   4, 11,  7,  4,  9, 11,  9, 10, 11, -1, -1, -1, -1, -1, -1, -1 ],
	[   0,  8,  3,  4,  9,  7,  9, 11,  7,  9, 10, 11, -1, -1, -1, -1 ],
	[   1, 10, 11,  1, 11,  4,  1,  4,  0,  7,  4, 11, -1, -1, -1, -1 ],
	[   3,  1,  4,  3,  4,  8,  1, 10,  4,  7,  4, 11, 10, 11,  4, -1 ],
	[   4, 11,  7,  9, 11,  4,  9,  2, 11,  9,  1,  2, -1, -1, -1, -1 ],
	[   9,  7,  4,  9, 11,  7,  9,  1, 11,  2, 11,  1,  0,  8,  3, -1 ],
	[  11,  7,  4, 11,  4,  2,  2,  4,  0, -1, -1, -1, -1, -1, -1, -1 ],
	[  11,  7,  4, 11,  4,  2,  8,  3,  4,  3,  2,  4, -1, -1, -1, -1 ],
	[   2,  9, 10,  2,  7,  9,  2,  3,  7,  7,  4,  9, -1, -1, -1, -1 ],
	[   9, 10,  7,  9,  7,  4, 10,  2,  7,  8,  7,  0,  2,  0,  7, -1 ],
	[   3,  7, 10,  3, 10,  2,  7,  4, 10,  1, 10,  0,  4,  0, 10, -1 ],
	[   1, 10,  2,  8,  7,  4, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   4,  9,  1,  4,  1,  7,  7,  1,  3, -1, -1, -1, -1, -1, -1, -1 ],
	[   4,  9,  1,  4,  1,  7,  0,  8,  1,  8,  7,  1, -1, -1, -1, -1 ],
	[   4,  0,  3,  7,  4,  3, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   4,  8,  7, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   9, 10,  8, 10, 11,  8, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   3,  0,  9,  3,  9, 11, 11,  9, 10, -1, -1, -1, -1, -1, -1, -1 ],
	[   0,  1, 10,  0, 10,  8,  8, 10, 11, -1, -1, -1, -1, -1, -1, -1 ],
	[   3,  1, 10, 11,  3, 10, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   1,  2, 11,  1, 11,  9,  9, 11,  8, -1, -1, -1, -1, -1, -1, -1 ],
	[   3,  0,  9,  3,  9, 11,  1,  2,  9,  2, 11,  9, -1, -1, -1, -1 ],
	[   0,  2, 11,  8,  0, 11, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   3,  2, 11, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   2,  3,  8,  2,  8, 10, 10,  8,  9, -1, -1, -1, -1, -1, -1, -1 ],
	[   9, 10,  2,  0,  9,  2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   2,  3,  8,  2,  8, 10,  0,  1,  8,  1, 10,  8, -1, -1, -1, -1 ],
	[   1, 10,  2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   1,  3,  8,  9,  1,  8, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   0,  9,  1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[   0,  3,  8, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
	[  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
];

const S_CUBE: [[f32; 3]; 8] =
[
	[ 0.0, 1.0, 1.0 ], // 0
	[ 1.0, 1.0, 1.0 ], // 1
	[ 1.0, 1.0, 0.0 ], // 2
	[ 0.0, 1.0, 0.0 ], // 3
	[ 0.0, 0.0, 1.0 ], // 4
	[ 1.0, 0.0, 1.0 ], // 5
	[ 1.0, 0.0, 0.0 ], // 6
	[ 0.0, 0.0, 0.0 ], // 7
];

const S_IDX1: [u32; 12] = [ 1, 2, 3, 0, 5, 6, 7, 4, 4, 5, 6, 7];

fn vert_lerp(result: &mut [f32; 6], iso: f32, idx0: usize, v0: f32, idx1: usize, v1: f32) -> f32 {

	let edge0 = &S_CUBE[idx0];
	let edge1 = &S_CUBE[idx1];

	if (iso - v1).abs() < 0.00001f32 {
		result[0] = edge1[0];
		result[1] = edge1[1];
		result[2] = edge1[2];
		return 1f32;
	}

	if (iso - v0).abs() < 0.00001f32
	|| ( v0 - v1).abs() < 0.00001f32 {
		result[0] = edge0[0];
		result[1] = edge0[1];
		result[2] = edge0[2];
		return 0.0f32;
	}

	let lerp = (iso - v0) / (v1 - v0);
	result[0] = edge0[0] + lerp * (edge1[0] - edge0[0]);
	result[1] = edge0[1] + lerp * (edge1[1] - edge0[1]);
	result[2] = edge0[2] + lerp * (edge1[2] - edge0[2]);

	lerp
}

fn triangulate(result: &mut [PosNormalColorVertex], rgb: &[f32], xyz: &[f32], val: &[&Grid; 8], iso: f32) -> usize {
	let mut cubeindex = 0;
	if val[0].val < iso { cubeindex |= 0x01; };
	if val[1].val < iso { cubeindex |= 0x02; };
	if val[2].val < iso { cubeindex |= 0x04; };
	if val[3].val < iso { cubeindex |= 0x08; };
	if val[4].val < iso { cubeindex |= 0x10; };
	if val[5].val < iso { cubeindex |= 0x20; };
	if val[6].val < iso { cubeindex |= 0x40; };
	if val[7].val < iso { cubeindex |= 0x80; };

	let flags: u16 = S_EDGES[cubeindex];
	if flags == 0 {
		return 0;
	}

	let mut verts = [[0f32; 6]; 12]; // Wasted initialization. Pass as parameter?

	for ii in 0..12 {
		if (flags & (1u16 << ii) ) != 0 {
			let idx0 = ii & 7;
			let idx1 = S_IDX1[ii] as usize;
			let vertex = &mut verts[ii];
			let lerp = vert_lerp(vertex, iso, idx0, val[idx0].val, idx1, val[idx1].val);

			let na = &val[idx0].normal;
			let nb = &val[idx1].normal;
			vertex[3] = na.x + lerp * (nb[0] - na.x);
			vertex[4] = na.y + lerp * (nb[1] - na.y);
			vertex[5] = na.z + lerp * (nb[2] - na.z);
		}
	}

	let dr = rgb[3] - rgb[0];
	let dg = rgb[4] - rgb[1];
	let db = rgb[5] - rgb[2];

	let mut num = 0;
	let indices = S_INDICES[cubeindex];
    
	while num < 16 {
        let ii = indices[num];
        if ii == -1 {
            break;
        }
		let vertex = verts[ii as usize];

		let res = &mut result[num];
		res._x = xyz[0] + vertex[0];
		res._y = xyz[1] + vertex[1];
		res._z = xyz[2] + vertex[2];

		res._nx = vertex[3];
		res._ny = vertex[4];
		res._nz = vertex[5];

		let rr = ( (rgb[0] + vertex[0]*dr)*255.0f32) as u32;
		let gg = ( (rgb[1] + vertex[1]*dg)*255.0f32) as u32;
		let bb = ( (rgb[2] + vertex[2]*db)*255.0f32) as u32;

		res._abgr = 0xff000000u32
			  | (bb << 16)
			  | (gg << 8)
			  | rr;
		num += 1;
	}

	return num;
}

const DIMS: usize = 32;

struct Metaballs<'a> {
    bgfx: &'a Bgfx,
    events: EventQueue,
    width: u16,
    height: u16,
    debug: DebugFlags,
    reset: ResetFlags,
    program: Option<Program<'a>>,
	decl: Option<VertexDecl>,
    grid: [Grid; DIMS*DIMS*DIMS],
    time: Option<PreciseTime>,
    last: Option<PreciseTime>,
}

impl<'a> Metaballs<'a> {

    #[inline]
    fn new(bgfx: &'a Bgfx, events: EventQueue) -> Self {
        Self {
            bgfx: bgfx,
            events: events,
            width: 0,
            height: 0,
            debug: DEBUG_NONE,
            reset: RESET_NONE,
            grid: [Default::default(); DIMS*DIMS*DIMS],
            program: None,
			decl: None,
            time: None,
            last: None,
        }
    }

    fn init(&mut self) {
        self.width = 1280;
        self.height = 720;
        self.debug = DEBUG_TEXT;
        self.reset = RESET_VSYNC;

        // This is where the C++ example would call bgfx::init(). In rust we move that out of this
        // object due to lifetimes: The Cubes type cannot own both the Bgfx object, and guarantee
        // that its members are destroyed before the Bgfx object.
        self.bgfx.reset(self.width, self.height, self.reset);

        // Enable debug text.
        self.bgfx.set_debug(self.debug);

        // Set view 0 clear state.
        let clear_flags = CLEAR_COLOR | CLEAR_DEPTH;
        self.bgfx.set_view_clear(0, clear_flags, 0x303030ff, 1.0_f32, 0);

        // Create vertex stream declaration
        self.decl = Some(PosNormalColorVertex::build_decl());

        // Create program from embedded shaders.
        // self.program = Some(common::create_program(&self.bgfx, VS_METABALLS_OPENGL, FS_METABALLS_OPENGL));
        self.program = Some(common::create_program(&self.bgfx, VS_METABALLS_D3D11, FS_METABALLS_D3D11));

        self.time = Some(PreciseTime::now());

        let caps = self.bgfx.caps();
        println!("{:#?}", caps);

    }

    fn shutdown(&mut self) {
        // We don't really need to do anything here, the objects will clean themselves up once they
        // go out of scope. This function is really only here to keep the examples similar in
        // structure to the C++ examples.
    }

    fn update(&mut self) -> bool {
        if !self.events.handle_events(&self.bgfx, &mut self.width, &mut self.height, self.reset) {

            let ypitch = DIMS;
            let zpitch = DIMS*DIMS;
            let invdim = 1.0f32/((DIMS-1) as f32);
#[allow(non_snake_case)]
			let fDIMS = DIMS as f32;

            // Set view 0 default viewport.
            self.bgfx.set_view_rect(0, 0, 0, self.width, self.height);

            // This dummy draw call is here to make sure that view 0 is cleared if no other draw
            // calls are submitted to view 0.
            self.bgfx.touch(0);

            let now = PreciseTime::now();
            let frame_time = self.last.unwrap_or(now).to(now);
            self.last = Some(now);

            let time = (self.time.unwrap().to(now).num_microseconds().unwrap() as f64) /
                       1_000_000.0_f64;

            // Use debug font to print information about this example.
            self.bgfx.dbg_text_clear(None, None);
            self.bgfx.dbg_text_print(0, 1, 0x4f, "examples/02-metaballs.rs");
            self.bgfx.dbg_text_print(0, 2, 0x6f, "Description: Rendering with transient buffers and embedding shaders.");

            let at = Point3::new(1.0, 2.0, 3.0);
            let eye = Point3::new(4.0, 5.0, -50.0);
            let up = Vector3::new(0.0, 1.0, 0.0);

            // TODO: Support for HMD rendering

	        let caps = self.bgfx.caps();
            // Set view and projection matrix for view 0.
            let aspect = (self.width as f32) / (self.height as f32);
			let mut view = Matrix4::look_at(eye, at, up);
			correct_view_matrix(&mut view);
			let mut proj = cgmath::perspective(Deg(60.0), aspect, 0.1, 100.0);
			correct_proj_matrix(&mut proj, caps.homogeneousDepth);

            self.bgfx.set_view_transform(0, view.as_ref(), proj.as_ref());

			// Allocate 32K vertices in transient vertex buffer.
			const MAX_VERTICES: usize = 32 << 10;
            if let Some(mut tvb) = self.bgfx.alloc_transient_vertex_buffer::<PosNormalColorVertex>(MAX_VERTICES, self.decl.as_ref().unwrap()) {

                // Simulate spheres
                const NUM_SPHERES: usize = 16;
                let mut spheres = [[0f32; 4]; NUM_SPHERES];
                for (ii, sphere) in spheres.iter_mut().enumerate() {
					let fi = ii as f32;
                    let ti = (time * ii as f64) as f32;
                    sphere[0] = (ti*0.21f32 + fi*0.37f32).sin() * (fDIMS * 0.5f32 - 8.0f32);
                    sphere[1] = (ti*0.37f32 + fi*0.67f32).sin() * (fDIMS * 0.5f32 - 8.0f32);
                    sphere[2] = (ti*0.11f32 + fi*0.13f32).cos() * (fDIMS * 0.5f32 - 8.0f32);
                    sphere[3] = 1.0f32/(2.0f32 + ((ti*0.13f32).sin() * 0.5f32 + 0.5f32)*2.0f32);
                }

                // Compute field
				let prof_update = PreciseTime::now();
                let halfdim = (DIMS as f32)*0.5f32;
                for zz in 0..DIMS {
                    for yy in 0..DIMS {
    					let offset = (zz*DIMS + yy)*DIMS;
                        for xx in 0..DIMS {
                            let xoffset = offset + xx;

                            let mut dist = 0.0f32;
                            let mut prod = 1.0f32;
                            for sphere in &spheres {
                                let dx = sphere[0] - (xx as f32 - halfdim);
                                let dy = sphere[1] - (yy as f32 - halfdim);
                                let dz = sphere[2] - (zz as f32 - halfdim);
                                let invr = sphere[3];
                                let dot = (dx*dx + dy*dy + dz*dz) * invr * invr;

                                dist *= dot;
                                dist += prod;
                                prod *= dot;
                            }

                            self.grid[xoffset].val = dist / prod - 1.0f32;
                        }
                    }
                }
				let prof_update = prof_update.to(PreciseTime::now()).num_microseconds().unwrap_or(0) as f32 / 1000f32;

                // Compute normals
				let prof_normal = PreciseTime::now();
                for zz in 1..(DIMS-1) {
                    for yy in 1..(DIMS-1) {
    					let offset = (zz*DIMS + yy)*DIMS;
                        for xx in 1..(DIMS-1) {
                            let xoffset = offset + xx;

                            let grid = &mut self.grid;
                            let normal = Vector3::new(
                                grid[xoffset-1     ].val - grid[xoffset+1     ].val,
                                grid[xoffset-ypitch].val - grid[xoffset+ypitch].val,
                                grid[xoffset-zpitch].val - grid[xoffset+zpitch].val,
                            );
                            grid[xoffset].normal = normal.normalize();
                        }
                    }
                }
				let prof_normal = prof_normal.to(PreciseTime::now()).num_microseconds().unwrap_or(0) as f32 / 1000f32;

                // Generate mesh
				let prof_triangulate = PreciseTime::now();
				let mut num_vertices: usize;
				{
					let vertex = &mut tvb.data;
					num_vertices = 0;
					let mut rgb = [0f32; 6];

					for zz in 0..(DIMS-1) {
						if num_vertices+12 >= MAX_VERTICES {
							break
						}
						let fzz = zz as f32;
						rgb[2] = fzz*invdim;
						rgb[5] = (fzz+1f32)*invdim;

						for yy in 0..(DIMS-1) {
							if num_vertices+12 >= MAX_VERTICES {
								break
							}
							let offset = (zz*DIMS+yy)*DIMS;

							let fyy = yy as f32;
							rgb[1] = fyy*invdim;
							rgb[4] = (fyy+1f32)*invdim;

							for xx in 0..(DIMS-1) {
								if num_vertices+12 >= MAX_VERTICES {
									break
								}
								let xoffset = offset + xx;

								let fxx = xx as f32;
								rgb[0] = fxx*invdim;
								rgb[3] = (fxx+1f32)*invdim;

								let pos = [
									-fDIMS*0.5f32 + fxx,
									-fDIMS*0.5f32 + fyy,
									-fDIMS*0.5f32 + fzz,
								];

								let grid = &self.grid;
								let val = [
									&grid[xoffset+zpitch+ypitch  ],
									&grid[xoffset+zpitch+ypitch+1],
									&grid[xoffset+ypitch+1       ],
									&grid[xoffset+ypitch         ],
									&grid[xoffset+zpitch         ],
									&grid[xoffset+zpitch+1       ],
									&grid[xoffset+1              ],
									&grid[xoffset                ],
								];

								let num = triangulate( &mut vertex[num_vertices..], &rgb, &pos, &val, 0.5);
								num_vertices += num;
							}
						}
					}
				}
				let prof_triangulate = prof_triangulate.to(PreciseTime::now()).num_microseconds().unwrap_or(0) as f32 / 1000f32;

                let mut modifier = Decomposed::one();
                modifier.rot = Quaternion::from(Euler::new(Rad(time * 0.67),
                                                    Rad(time),
                                                    Rad(0.0)));
                let mut mtx = Matrix4::from(modifier).cast::<f32>();
				correct_model_matrix(&mut mtx);

                // Set model matrix for rendering.
                self.bgfx.set_transform(mtx.as_ref());

                // Set vertex buffer
                self.bgfx.set_transient_vertex_buffer_partial(&tvb, 0, num_vertices);

                // Set render states.
                self.bgfx.set_state(STATE_DEFAULT, None);

                // Submit primitive for rendering to view 0.
                self.bgfx.submit(0, self.program.as_ref().unwrap());

				// Display stats.
				self.bgfx.dbg_text_print(1, 4, 0x0f, &format!("Num vertices: {:5.3} ({:5.1}%)", num_vertices, num_vertices as f32 / MAX_VERTICES as f32 * 100f32));
				self.bgfx.dbg_text_print(1, 5, 0x0f, &format!("      Update: {:7.3}ms", prof_update));
				self.bgfx.dbg_text_print(1, 6, 0x0f, &format!("Calc normals: {:7.3}ms", prof_normal));
				self.bgfx.dbg_text_print(1, 7, 0x0f, &format!(" Triangulate: {:7.3}ms", prof_triangulate));
				self.bgfx.dbg_text_print(1, 8, 0x0f, &format!("       Frame: {:7.3}ms", frame_time.num_milliseconds()));


            } else {
				println!("Failed to allocate the transient");
			}

            // Advance to next frame. Rendering thread will be kicked to process submitted
            // rendering primitives.
            self.bgfx.frame();

            true
        } else {
            false
        }
    }
}

fn example(events: EventQueue) {
    // let bgfx = bgfx::init(RendererType::OpenGL, None, None).unwrap();
    let bgfx = bgfx::init(RendererType::Direct3D11, None, None).unwrap();
    let mut metaballs = Metaballs::new(&bgfx, events);
    metaballs.init();
    while metaballs.update() {}
    metaballs.shutdown();
}

fn main() {
    common::run_example(1280, 720, example);
}
