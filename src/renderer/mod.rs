use std::cell::{Ref, RefMut};

use bevy_ecs::system::Resource;
use nalgebra::{Matrix4, Projective3, Rotation3, Transform, Translation, Vector3, Vector4};
use raylib::{
    core::texture::RenderTexture2D,
    ffi::{
        rlDrawRenderBatchActive, rlLoadIdentity, rlMatrixMode, rlPopMatrix, rlPushMatrix,
        rlSetMatrixModelview, rlSetMatrixProjection, TraceLog, RL_CULL_DISTANCE_FAR,
        RL_CULL_DISTANCE_NEAR, RL_PROJECTION,
    },
    prelude::*,
};

use crate::{components::Player, Cube, Position};

#[derive(Clone, Copy)]
pub enum RenderCommand {
    Cube { position: Position, cube: Cube },
    CubeWithWires { position: Position, cube: Cube },
}

#[derive(Clone, Copy, Default)]
pub enum RenderMask {
    // Only render for a specific player.
    Whitelist(u32),
    // Don't render for a specific player.
    Blacklist(u32),
    // Always render
    #[default]
    None,
}

#[derive(Clone, Default, Resource)]
pub struct RaylibRenderer {
    pub commands: Vec<(RenderCommand, RenderMask)>,
}

impl RaylibRenderer {
    pub fn draw_command(command: &RenderCommand, d: &RefMut<'_, RaylibDrawHandle<'_>>) {
        match command {
            RenderCommand::Cube { position, cube } => {
                d.draw_cube_v(position.0.into(), cube.size.into(), cube.color)
            }
            RenderCommand::CubeWithWires { position, cube } => {
                d.draw_cube_v(position.0.into(), cube.size.into(), cube.color);
                d.draw_cube_wires(
                    position.0.into(),
                    cube.size.x + 0.01f32,
                    cube.size.y + 0.01f32,
                    cube.size.z + 0.01f32,
                    Color::BLACK,
                );
            }
        }
    }

    pub fn render<'bind>(&self, d: &RefMut<RaylibDrawHandle<'bind>>, player_id: u32) {
        self.commands.iter().for_each(|(command, mask)| {
            let do_render = match mask {
                &RenderMask::Whitelist(mask_id) => player_id == mask_id,
                &RenderMask::Blacklist(mask_id) => player_id != mask_id,
                &RenderMask::None => true,
            };

            if do_render {
                Self::draw_command(command, d);
            }
        })
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }
}

pub struct PlayerRenderState<'bind, 'a> {
    pub player: Player,
    pub framebuffer: RenderTexture2D<'bind, 'a>,
}

pub struct RenderState<'bind, 'a> {
    pub players: [PlayerRenderState<'bind, 'a>; 2],
}

impl<'bind, 'a> RenderState<'bind, 'a> {
    pub fn new(rl: &'bind RaylibHandle, t: &RaylibThread, players: [Player; 2]) -> Self {
        Self {
            players: [
                PlayerRenderState {
                    player: players[0],
                    framebuffer: rl
                        .load_render_texture(
                            t,
                            (rl.get_screen_width() / 2) as _,
                            rl.get_screen_height() as _,
                        )
                        .expect("Unable to create RenderTexture"),
                },
                PlayerRenderState {
                    player: players[1],
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

    fn get_player_modelview_matrix(player: &Player) -> Matrix4<f32> {
        Matrix4::new_translation(&Vector3::new(player.position.x, player.height, player.position.y))
            * Matrix4::from_euler_angles(0f32, 0f32, player.rotation.angle())
    }

    fn begin_player<F>(&self, d: Ref<RaylibHandle<'bind>>, player_id: usize, f: F)
    where
        F: FnOnce(Ref<RaylibHandle<'bind>>),
    {
        let Some(render_state) = self.players.get(player_id) else {
            eprintln!("Unknown player {player_id}");
            return;
        };

        // SAFE: Raylib is initialized (`d` existence requirement).
        unsafe {
            rlDrawRenderBatchActive();

            rlMatrixMode(RL_PROJECTION as _);
            rlPushMatrix();
            rlSetMatrixProjection(
                Matrix4::new_perspective(
                    8f32 / 9f32, // hardcoded
                    70,
                    RL_CULL_DISTANCE_NEAR,
                    RL_CULL_DISTANCE_FAR,
                )
                .into(),
            );
            rlSetMatrixModelview(Self::get_player_modelview_matrix(&render_state.player).into());
        }

        unsafe {
            rlPopMatrix();
        }
    }
}
