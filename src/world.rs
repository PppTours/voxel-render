use bevy_ecs::world::World;
use raylib::ffi::{Camera3D, CameraProjection, Color, Vector3};

use crate::components::{Cube, Player, Position};

pub fn init_world(world: &mut World) -> (Player, Player) {
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

    (player0, player1)
}
