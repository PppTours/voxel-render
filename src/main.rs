pub mod components;
pub mod renderer;
pub mod systems;

use bevy_ecs::prelude::*;
use raylib::{
    core::texture::RenderTexture2D,
    ffi::{mint::Vector3, Camera3D, CameraProjection},
    prelude::*,
};

use components::{Cube, GameInput, Player, Position};
use renderer::RaylibRenderer;
use systems::{display_cube, update_player_camera};

pub struct PlayerRenderState<'bind, 'a> {
    camera: Camera3D,
    framebuffer: RenderTexture2D<'bind, 'a>,
}

pub struct RenderState<'bind, 'a> {
    players: [PlayerRenderState<'bind, 'a>; 2],
}

impl<'bind, 'a> RenderState<'bind, 'a> {
    pub fn init(rl: &'bind RaylibHandle, t: &RaylibThread, cameras: [Camera3D; 2]) -> Self {
        Self {
            players: [
                PlayerRenderState {
                    camera: cameras[0],
                    framebuffer: rl
                        .load_render_texture(
                            t,
                            (rl.get_screen_width() / 2) as _,
                            rl.get_screen_height() as _,
                        )
                        .expect("Unable to create RenderTexture"),
                },
                PlayerRenderState {
                    camera: cameras[1],
                    framebuffer: rl
                        .load_render_texture(
                            t,
                            (rl.get_screen_width() / 2) as _,
                            rl.get_screen_height() as _,
                        )
                        .expect("Unable to create RenderTexture"),
                },
            ],
        }
    }
}

fn main() {
    let (rl, t) = raylib::init().width(1280).height(800).vsync().build();
    rl.disable_cursor();

    let mut world = World::new();
    world.insert_resource(RaylibRenderer::default());
    world.insert_resource(GameInput::default());

    let mut schedule = Schedule::default();
    schedule.add_systems(display_cube);
    schedule.add_systems(update_player_camera);

    let player0 = Player {
        camera: Camera3D {
            position: Vector3 {
                x: 0f32,
                y: 0f32,
                z: 0f32,
            },
            target: Vector3 {
                x: 1f32,
                y: 1f32,
                z: 1f32,
            },
            up: Vector3 {
                x: 0f32,
                y: 1f32,
                z: 0f32,
            },
            fovy: 70f32,
            projection: CameraProjection::CAMERA_PERSPECTIVE as _,
        },
        id: 0,
    };
    world.spawn((
        player0,
        Position(player0.camera.position),
        Cube {
            color: Color::BLUE,
            size: Vector3 {
                x: 1f32,
                y: 1f32,
                z: 1f32,
            },
        },
    ));

    let mut player1 = player0.clone();
    player1.id = 1;

    world.spawn((
        player1,
        Position(player1.camera.position),
        Cube {
            color: Color::YELLOW,
            size: Vector3 {
                x: 1f32,
                y: 1f32,
                z: 1f32,
            },
        },
    ));

    world.spawn((
        Position(Vector3 {
            x: 1f32,
            y: 1f32,
            z: 1f32,
        }),
        Cube {
            color: Color::RED,
            size: Vector3 {
                x: 1f32,
                y: 1f32,
                z: 1f32,
            },
        },
    ));

    let mut render_state = RenderState::init(&rl, &t, [player0.camera, player1.camera]);

    while !rl.window_should_close() {
        let mut input: Mut<GameInput> = world.get_resource_mut().unwrap();
        input.update_inputs(&rl);

        schedule.run(&mut world);

        for entity in world.query_filtered::<Entity, &Player>().iter(&world) {
            let player: &Player = world.get(entity).unwrap();
            render_state.players[player.id as usize].camera = player.camera;
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
