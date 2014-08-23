use gfx;
use device;
use piston;
use Window;

use gfx::{Device, DeviceHelper};

use shader_source;
use shader_param::Program;

pub struct Graphics {
    device: device::gl::GlDevice,
    renderer: gfx::Renderer,
    frame: gfx::Frame,
    background_color: [f32, ..4],
    program: Program,
}

impl Graphics {
    pub fn from_window(window: &mut Window) -> Graphics {
        let (mut device, frame) = window.gfx();
        let renderer = device.create_renderer();
        let program = device.link_program(
                shader_source::VERTEX_SRC.clone(),
                shader_source::FRAGMENT_SRC.clone()
            ).unwrap();
        Graphics {
            device: device,
            renderer: renderer,
            frame: frame,
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
                color: Some(gfx::Color(*background_color)),
                depth: Some(1.0),
                stencil: None,
            },
            frame
        );
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

