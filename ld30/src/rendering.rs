use gfx;
use device;
use piston;
use Window;

use gfx::{Device, DeviceHelper};

use shader_source;
use shader_param::Program;
use data;

pub struct Graphics {
    device: device::gl::GlDevice,
    renderer: gfx::Renderer,
    frame: gfx::Frame,
    state: gfx::DrawState,
    vertices: gfx::Mesh,
    background_color: [f32, ..4],
    program: Program,
}

impl Graphics {
    pub fn new(window: &mut Window, data: &data::Data) -> Graphics {
        let (mut device, frame) = window.gfx();
        let renderer = device.create_renderer();
        let state = gfx::DrawState::new().depth(gfx::state::LessEqual, true);
        let program = device.link_program(
                shader_source::VERTEX_SRC.clone(),
                shader_source::FRAGMENT_SRC.clone()
            ).unwrap();
        let vertices = device.create_mesh(data.vertices.clone());
        Graphics {
            device: device,
            renderer: renderer,
            frame: frame,
            state: state,
            vertices: vertices,
            background_color: [0.0, 0.0, 0.0, 1.0],
            program: program,
        }
    }

    pub fn clear(&mut self) {
        let &Graphics {
            ref mut renderer,
            ref background_color,
            ref frame,
            ..
        } = self;
        renderer.reset();
        renderer.clear(
            gfx::ClearData {
                color: Some(*background_color),
                depth: Some(1.0),
                stencil: None,
            },
            frame
        );
    }

    pub fn draw_instance(&mut self, ty: data::Type, data: &data::Data) {
        let obj = match data.obj_data[ty.to_uint()] {
                Some(ref obj) => obj,
                None => return
            };
        for obj in obj.objects.iter() {
            for geom in obj.geometry.iter() {
                
            }
        }
    }

    pub fn flush(&mut self) {
        let &Graphics {
            ref renderer,
            ref mut device,
            ..
        } = self;
        device.submit(renderer.as_buffer());
    }
}

