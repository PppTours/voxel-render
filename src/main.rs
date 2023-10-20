pub mod components;
pub mod renderer;
pub mod systems;
pub mod world;

use bevy_ecs::prelude::*;
use raylib::prelude::*;

use components::{Cube, DeltaTime, GameInput, Player, Position};
use renderer::{RaylibRenderer, RenderState};
use systems::{display_cube, update_player_camera};
use world::init_world;

fn main() {
    let (rl, t) = raylib::init()
        .msaa_4x()
        .title("polyfps")
        //.vsync()
        .size(1280, 720)
        .build();
    rl.disable_cursor();

    let mut world = World::new();
    world.insert_resource(RaylibRenderer::default());
    world.insert_resource(GameInput::default());
    world.insert_resource(DeltaTime::default());

    let mut schedule = Schedule::default();
    schedule.add_systems((display_cube, update_player_camera));

    let (player0, player1) = init_world(&mut world);

    let mut render_state = RenderState::new(&rl, &t, [player0, player1]);

    while !rl.window_should_close() {
        world
            .get_resource_mut::<GameInput>()
            .unwrap()
            .update_inputs(&rl);
        world.get_resource_mut::<DeltaTime>().unwrap().0 = rl.get_frame_time();

        schedule.run(&mut world);

        for entity in world.query_filtered::<Entity, &Player>().iter(&world) {
            let player: &Player = world.get(entity).unwrap();
            render_state.players[player.id as usize].player = player.clone();
        }

        let mut renderer: Mut<RaylibRenderer> = world.get_resource_mut().unwrap();

        rl.begin_drawing(&t, |d| {
            d.clear_background(Color::SKYBLUE);

            // Render players viewpoints
            for (player_id, player_render_state) in render_state.players.iter_mut().enumerate() {
                d.begin_texture(&mut player_render_state.framebuffer, || {
                    d.clear_background(Color::SKYBLUE);

                    d.begin_camera_3d(&player_render_state.camera, || {
                        renderer.render(&d, player_id as u32)
                    });
                });

                if player_id == 0 {
                    d.draw_texture(&player_render_state.framebuffer, 0, 0, Color::WHITE);
                } else if player_id == 1 {
                    d.draw_texture(
                        &player_render_state.framebuffer,
                        rl.get_screen_width() / 2,
                        0,
                        Color::WHITE,
                    );
                }
            }

            d.draw_fps(10, 10);
        });

        renderer.clear();
    }
}
