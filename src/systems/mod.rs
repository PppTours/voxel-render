use std::borrow::BorrowMut;

use bevy_ecs::{
    entity::Entity,
    system::{Query, Res, ResMut},
};

use crate::{
    components::{Cube, GameInput, Player, Position, DeltaTime},
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

pub fn update_player_camera(
    input: Res<GameInput>,
    dt: Res<DeltaTime>,
    mut query: Query<(Entity, &mut Position, &mut Player)>,
) {
    for (_, mut position, mut player) in &mut query {
        if let Some(player_input) = input.player.get(player.id as usize) {
            println!("{player:?}");
            if player_input.move_forward {
                player.camera.move_forward(3f32 * dt.0, true);
            }

            if player_input.move_backward {
                player.camera.move_forward(-3f32 * dt.0, true);
            }

            if player_input.move_right {
                player.camera.move_right(3f32 * dt.0, true);
            }

            if player_input.move_left {
                player.camera.move_right(-3f32 * dt.0, true);
            }

            if player_input.view_up {
                player.camera.pitch(-3f32 * dt.0, true, false, false);
            }

            if player_input.view_down {
                player.camera.pitch(3f32 * dt.0, true, false, false);
            }

            if player_input.view_right {
                player.camera.yaw(-3f32 * dt.0, false);
            }

            if player_input.view_left {
                player.camera.yaw(3f32 * dt.0, false);
            }

            position.0 = player.camera.position;
        }
    }
}
