// Copyright (c) 2015-2016, Johan Sköld.
// Copyright (c) 2017, Javier Arevalo
// License: http://opensource.org/licenses/ISC

extern crate bgfx;
extern crate cgmath;
extern crate glutin;
extern crate time;

mod common;

use bgfx::*;
use cgmath::{Decomposed, Deg, Matrix4, Point3, Quaternion, Rad, Transform, Vector3, Vector4, Euler};
use common::*;
use time::PreciseTime;


#[repr(packed)]
struct PosColorVertex {
    _x: f32,
    _y: f32,
    _z: f32,
    _abgr: u32,
}

impl PosColorVertex {
    fn build_decl() -> VertexDecl {
        VertexDecl::new(None)
            .add(Attrib::Position, 3, AttribType::Float)
            .add(Attrib::Color0, 4, AttribType::Uint8(true))
            .end()
    }
}

#[repr(C)]
struct InstanceData {
    mtx: Matrix4<f32>,
    color: Vector4<f32>
}

#[cfg_attr(rustfmt, rustfmt_skip)]
static CUBE_VERTICES: [PosColorVertex; 8] = [
    PosColorVertex { _x: -1.0, _y:  1.0, _z:  1.0, _abgr: 0xff000000 },
    PosColorVertex { _x:  1.0, _y:  1.0, _z:  1.0, _abgr: 0xff0000ff },
    PosColorVertex { _x: -1.0, _y: -1.0, _z:  1.0, _abgr: 0xff00ff00 },
    PosColorVertex { _x:  1.0, _y: -1.0, _z:  1.0, _abgr: 0xff00ffff },
    PosColorVertex { _x: -1.0, _y:  1.0, _z: -1.0, _abgr: 0xffff0000 },
    PosColorVertex { _x:  1.0, _y:  1.0, _z: -1.0, _abgr: 0xffff00ff },
    PosColorVertex { _x: -1.0, _y: -1.0, _z: -1.0, _abgr: 0xffffff00 },
    PosColorVertex { _x:  1.0, _y: -1.0, _z: -1.0, _abgr: 0xffffffff },
];

#[cfg_attr(rustfmt, rustfmt_skip)]
static CUBE_INDICES: [u16; 36] = [
    0, 1, 2, // 0
    1, 3, 2,
    4, 6, 5, // 2
    5, 6, 7,
    0, 2, 4, // 4
    4, 2, 6,
    1, 5, 3, // 6
    5, 7, 3,
    0, 4, 1, // 8
    4, 5, 1,
    2, 3, 6, // 10
    6, 3, 7,
];

struct Cubes<'a> {
    bgfx: &'a Bgfx,
    events: EventQueue,
    width: u16,
    height: u16,
    debug: DebugFlags,
    reset: ResetFlags,
    vbh: Option<VertexBuffer<'a>>,
    ibh: Option<IndexBuffer<'a>>,
    program: Option<Program<'a>>,
    time: Option<PreciseTime>,
    last: Option<PreciseTime>,
}

impl<'a> Cubes<'a> {

    #[inline]
    fn new(bgfx: &'a Bgfx, events: EventQueue) -> Cubes<'a> {
        Cubes {
            bgfx: bgfx,
            events: events,
            width: 0,
            height: 0,
            debug: DEBUG_NONE,
            reset: RESET_NONE,
            vbh: None,
            ibh: None,
            program: None,
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
        let decl = PosColorVertex::build_decl();

        // Create static vertex buffer.
        self.vbh = Some(VertexBuffer::new(Memory::reference(self.bgfx, &CUBE_VERTICES),
                                          &decl,
                                          BUFFER_NONE));

        // Create static index buffer.
        self.ibh = Some(IndexBuffer::new(Memory::reference(self.bgfx, &CUBE_INDICES), BUFFER_NONE));

        // Create program from shaders.
        self.program = Some(common::load_program(&self.bgfx, "vs_instancing", "fs_instancing"));

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
            let frame_info = format!("Frame: {:7.3}[ms]", frame_time.num_milliseconds());
            self.bgfx.dbg_text_clear(None, None);
            self.bgfx.dbg_text_print(0, 1, 0x4f, "examples/05-instancing.rs");
            self.bgfx.dbg_text_print(0, 2, 0x6f, "Description: Geometry instancing.");
            self.bgfx.dbg_text_print(0, 3, 0x0f, &frame_info);

            let caps = self.bgfx.caps();

            if !caps.supported.contains(CAPS_INSTANCING) {
                let blink = if 0 != ((time*3.0) as u32 & 1) { 0x1f } else { 0x01 };
                self.bgfx.dbg_text_print(0, 5, blink, " Instancing is not supported by GPU. ");
            } else {

                let at = Point3::new(0.0, 0.0, 0.0);
                let eye = Point3::new(0.0, 0.0, -35.0);
                let up = Vector3::new(0.0, 1.0, 0.0);

                // TODO: Support for HMD rendering

                // Set view and projection matrix for view 0.
                let aspect = (self.width as f32) / (self.height as f32);
                let mut view = Matrix4::look_at(eye, at, up);
                correct_view_matrix(&mut view);
                let mut proj = cgmath::perspective(Deg(60.0), aspect, 0.1, 100.0);
                correct_proj_matrix(&mut proj, caps.homogeneousDepth);

                self.bgfx.set_view_transform(0, view.as_ref(), proj.as_ref());

				if let Some(idb) = self.bgfx.alloc_instance_data_buffer::<InstanceData>(11*11) {

                    // Submit 11x11 cubes
                    let mut i = 0;
                    for yy in 0..11 {
                        for xx in 0..11 {
                            let mut modifier = Decomposed::one();
                            modifier.rot = Quaternion::from(Euler::new(Rad((time + (xx as f64) * 0.21)),
                                                                Rad((time + (yy as f64) * 0.37)),
                                                                Rad(0.0)));
                            modifier.disp = Vector3::new(-15.0 + (xx as f64) * 3.0,
                                                        -15.0 + (yy as f64) * 3.0,
                                                        0.0);
                            idb.data[i] = InstanceData {
                                mtx: Matrix4::from(modifier).cast::<f32>(),
                                color: Vector4::new(((time+(xx as f64)/11.0).sin() as f32)*0.5f32 + 0.5f32,
                                                    ((time+(yy as f64)/11.0).cos() as f32)*0.5f32 + 0.5f32,
                                                    ((time*3.0).sin() as f32)*0.5f32 + 0.5f32,
                                                    1f32)
                            };
                            correct_model_matrix(&mut idb.data[i].mtx);
                            i += 1;
                        }
                    }

                    // Set vertex and index buffer.
                    self.bgfx.set_vertex_buffer(self.vbh.as_ref().unwrap());
                    self.bgfx.set_index_buffer(self.ibh.as_ref().unwrap());

                    // Set instance data buffer.
                    self.bgfx.set_instance_data_buffer(idb);

                    // Set render states.
                    self.bgfx.set_state(STATE_DEFAULT, None);

                    // Submit primitive for rendering to view 0.
                    self.bgfx.submit(0, self.program.as_ref().unwrap());
                }
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
    let bgfx = bgfx::init(RendererType::Default, None, None).unwrap();
    let mut cubes = Cubes::new(&bgfx, events);
    cubes.init();
    while cubes.update() {}
    cubes.shutdown();
}

fn main() {
    common::run_example(1280, 720, example);
}
