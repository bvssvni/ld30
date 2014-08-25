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

use piston::cam;

mod vertex;
mod shader_param;
mod shader_source;
mod rendering;
mod data;
mod logic;

fn main() {
    let asset_store = piston::AssetStore::from_folder("../bin/assets");
    let data = data::Data::from_asset_store(&asset_store);   
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

    let projection = cam::CameraPerspective {
            fov: 70.0f32,
            near_clip: 0.1,
            far_clip: 1000.0,
            aspect_ratio: 1.0
        }.projection();
    let _initial_cam_pos = [-1.967394f32, 1.608332, 2.264971];
    let mut first_person = cam::FirstPerson::new(
            _initial_cam_pos,
            cam::FirstPersonSettings::default()
        );
    let mut graphics = rendering::Graphics::new(&mut window, &data); 
    let game_iter_settings = piston::GameIteratorSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };

    for e in piston::GameIterator::new(&mut window, &game_iter_settings) {
        match e {
            piston::Render(_args) => {
                    graphics.clear();

                    let model = piston::vecmath::mat4_id();
                    let mvp = cam::model_view_projection(model, first_person.camera(0.0).orthogonal(),
                                                         projection);
                    let color = [1.0, 0.0, 0.0, 1.0];
                    graphics.draw_instance(data::Slab, mvp, color, &data);

                    graphics.flush();
            }
            piston::Update(args) => {
                // Update physics.
                world.step(args.dt as f32);
                first_person.update(args.dt);
            }
            piston::Input(ref args) => {
                first_person.input(args);

                // TEST
                let [x, y, z] = first_person.position;
                println!("{} {} {}", x, y, z);
            }
        }
    }
}

