#![feature(phase)]

extern crate piston;
extern crate gfx;
extern crate device;
extern crate nphysics3df32;
extern crate sdl2_game_window;
#[phase(plugin)]
extern crate gfx_macros;
extern crate wobj = "wavefront-obj";

use nphysics3df32 as phys;

pub use sdl2_game_window::GameWindowSDL2 as Window;

mod internal;
mod vertex;
mod shader_param;
mod shader_source;
mod rendering;

fn main() {
    let asset_store = piston::AssetStore::from_folder("../bin/assets");
    
    // TEST
    {
        use std::io;

        let slab = io::File::open(&asset_store.path("slab.obj").unwrap()).read_to_string().unwrap();
        let objects = wobj::obj::parse(slab);
    }

    let mut world = phys::world::World::new();
    let mut window = Window::new(
        piston::shader_version::opengl::OpenGL_3_2,
        piston::GameWindowSettings {
            title: "Ludum Dare 30".to_string(),
            size: [640, 480],
            exit_on_esc: true,
            fullscreen: false,
        }
    );
   
    let mut graphics = rendering::Graphics::from_window(&mut window); 
    let game_iter_settings = piston::GameIteratorSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };
    for e in piston::GameIterator::new(&mut window, &game_iter_settings) {
        match e {
            piston::Render(_args) => {
                    graphics.clear();
                    graphics.flush();
                },
            piston::Update(args) => {
                // Update physics.
                world.step(args.dt as f32);
            },
            piston::Input(_args) => {}
        }
    }
}

