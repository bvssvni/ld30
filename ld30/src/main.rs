
extern crate piston;
extern crate gfx;
extern crate nphysics3df32;
extern crate sdl2_game_window;

use nphysics3df32 as phys;
use sdl2_game_window::GameWindowSDL2 as Window;

fn main() {
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

    let game_iter_settings = piston::GameIteratorSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };
    for e in piston::GameIterator::new(&mut window, &game_iter_settings) {
        match e {
            piston::Render(_args) => {},
            piston::Update(args) => {
                // Update physics.
                world.step(args.dt as f32);
            },
            piston::Input(_args) => {}
        }
    }
}

