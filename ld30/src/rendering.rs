use gfx;
use device;
use Window;
use piston::vecmath::Matrix4;

use gfx::{Device, DeviceHelper};

use shader_source;
use shader_param::{Program, ShaderParam};
use data;

pub struct Graphics {
    device: device::gl::GlDevice,
    renderer: gfx::Renderer,
    frame: gfx::Frame,
    state: gfx::DrawState,
    vertices: gfx::Mesh,
    indices: gfx::BufferHandle<u32>,
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
        let indices = device.create_buffer_static(&data.indices);
        Graphics {
            device: device,
            renderer: renderer,
            frame: frame,
            state: state,
            vertices: vertices,
            indices: indices,
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

    pub fn draw_instance(
        &mut self, 
        ty: data::Type, 
        mat: Matrix4<f32>, 
        color: [f32, ..4], 
        data: &data::Data
    ) {
        data.with_type_index_ranges(ty, |start, end| {
            let ref mesh = self.vertices;
            let slice = gfx::IndexSlice32(gfx::TriangleList, self.indices, start as u32, end as u32);
            let ref frame = self.frame;
            let ref prog = self.program;
            let shader_param = ShaderParam {
                model_view_projection: mat,
                color: color, 
            };
            let ref state = self.state;
            self.renderer.draw(mesh, slice, frame, (prog, &shader_param), state).unwrap();
        }); 
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

