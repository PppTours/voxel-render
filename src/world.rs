use bevy_ecs::world::World;
use nalgebra::{Rotation, Vector2, Vector3};
use raylib::ffi::Color;

use crate::components::{Cube, Player, Position};

pub fn init_world(world: &mut World) -> (Player, Player) {
    let player0 = Player {
        position: Vector2::zeros(),
        id: 0,
        height: 0f32,
        rotation: Rotation::default(),
        velocity: Vector3::zeros(),
    };
    world.spawn((
        player0,
        Position(Vector3::new(player0.position.x, 0f32, player0.position.y)),
        Cube {
            color: Color::BLUE,
            size: Vector3::new(1f32, 1f32, 1f32),
        },
    ));

    let mut player1 = player0.clone();
    player1.id = 1;

    world.spawn((
        player1,
        Position(Vector3::new(player1.position.x, 0f32, player1.position.y)),
        Cube {
            color: Color::YELLOW,
            size: Vector3::new(1f32, 1f32, 1f32),
        },
    ));

    world.spawn((
        Position(Vector3::new(1f32, 1f32, 1f32)),
        Cube {
            color: Color::RED,
            size: Vector3::new(1f32, 1f32, 1f32),
        },
    ));

    (player0, player1)
}
