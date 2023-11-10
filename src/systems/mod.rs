use std::{borrow::BorrowMut, f32::consts::PI};

use bevy_ecs::{
    entity::Entity,
    system::{Query, Res, ResMut},
};
use nalgebra::{Rotation2, Vector2, Vector3};

use crate::{
    components::{Cube, DeltaTime, GameInput, Player, Position},
    renderer::{RaylibRenderer, RenderCommand, RenderMask},
};

pub fn display_cube(
    mut renderer: ResMut<RaylibRenderer>,
    query: Query<(Entity, &Position, &Cube, Option<&Player>)>,
) {
    for (_, &position, &cube, player) in &query {
        renderer.borrow_mut().commands.push((
            RenderCommand::CubeWithWires { position, cube },
            match player {
                Some(player) => RenderMask::Blacklist(player.id),
                None => RenderMask::None,
            },
        ));
    }
}

const UP_VECTOR: Vector2<f32> = Vector2::new(0.0, -1.0);
const PLAYER_SPEED: f32 = 2.5;
const ROTATION_SENSIVITY: f32 = 1.5;

pub fn update_player_camera(
    input: Res<GameInput>,
    dt: Res<DeltaTime>,
    mut query: Query<(Entity, &mut Player, &mut Position)>,
) {
    for (_, mut player, mut world_position) in &mut query {
        if let Some(player_input) = input.player.get(player.id as usize) {
            println!("{player:?}");
            let mut new_pos = player.position.clone();

            if player_input.move_forward {
                new_pos += PLAYER_SPEED * &player.rotation.transform_vector(&UP_VECTOR) * dt.0;
            }

            if player_input.move_backward {
                new_pos -= PLAYER_SPEED * &player.rotation.transform_vector(&UP_VECTOR) * dt.0;
            }

            if player_input.move_right {
                new_pos += PLAYER_SPEED
                    * (&player.rotation * Rotation2::new(PI / 2.0)).transform_vector(&UP_VECTOR)
                    * dt.0;
            }

            if player_input.move_left {
                new_pos += PLAYER_SPEED
                    * (&player.rotation * Rotation2::new(-PI / 2.0)).transform_vector(&UP_VECTOR)
                    * dt.0;
            }

            if player_input.view_up {
                //player.camera.pitch(-3f32 * dt.0, true, false, false);
            }

            if player_input.view_down {
                //player.camera.pitch(3f32 * dt.0, true, false, false);
            }

            if player_input.view_right {
                player.rotation *= Rotation2::new(ROTATION_SENSIVITY * dt.0 * PI / 2.0);
                //player.camera.yaw(-3f32 * dt.0, false);
            }

            if player_input.view_left {
                //player.camera.yaw(3f32 * dt.0, false);
                player.rotation *= Rotation2::new(ROTATION_SENSIVITY * dt.0 * -PI / 2.0);
            }

            player.position = new_pos;
            world_position.0 = Vector3::new(player.position.x, player.height, player.position.y);
        }
    }
}
