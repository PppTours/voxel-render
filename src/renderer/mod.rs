use std::cell::RefMut;

use bevy_ecs::system::Resource;
use nalgebra::{Matrix4, Vector3};
use raylib::{
    core::texture::RenderTexture2D,
    ffi::{
        mint::ColumnMatrix4, rlDrawRenderBatchActive, rlEnableDepthTest, rlMatrixMode, rlPopMatrix,
        rlPushMatrix, rlSetMatrixModelview, rlSetMatrixProjection, Matrix, DEG2RAD,
        RL_CULL_DISTANCE_FAR, RL_CULL_DISTANCE_NEAR, RL_MODELVIEW, RL_PROJECTION,
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
    pub fn draw_command(command: &RenderCommand, d: &RefMut<'_, RaylibDrawHandle>) {
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

    pub fn render<'bind>(&self, d: &RefMut<RaylibDrawHandle>, player_id: u32) {
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

pub struct PlayerRenderState<'rl> {
    pub player: Player,
    pub framebuffer: RenderTexture2D<'rl>,
}

pub struct RenderState<'rl> {
    pub players: [PlayerRenderState<'rl>; 2],
    fb_width: i32,
}

const CAMERA_FOVY: f32 = 70.0;
const CAMERA_ASPECT: f32 = 8f32 / 9f32; // 16/9 / 2

impl<'rl> RenderState<'rl> {
    pub fn new(rl: &'rl RaylibHandle, players: [Player; 2]) -> Self {
        Self {
            players: [
                PlayerRenderState {
                    player: players[0],
                    framebuffer: rl
                        .load_render_texture(
                            (rl.get_screen_width() / 2) as _,
                            rl.get_screen_height() as _,
                        )
                        .expect("Unable to create RenderTexture"),
                },
                PlayerRenderState {
                    player: players[1],
                    framebuffer: rl
                        .load_render_texture(
                            (rl.get_screen_width() / 2) as _,
                            rl.get_screen_height() as _,
                        )
                        .expect("Unable to create RenderTexture"),
                },
            ],
            fb_width: rl.get_screen_width() / 2,
        }
    }

    fn get_player_modelview_matrix(player: &Player) -> Matrix4<f32> {
        Matrix4::from_euler_angles(0.0, player.rotation.angle(), 0.0).prepend_translation(
            &Vector3::new(player.position.x, player.height, player.position.y),
        )
    }

    pub fn begin_player<F, G>(
        &mut self,
        d: &RefMut<RaylibDrawHandle>,
        player_id: u32,
        ui: F,
        world: G,
    ) where
        F: FnOnce(&RefMut<RaylibDrawHandle>),
        G: FnOnce(&RefMut<RaylibDrawHandle>),
    {
        let player_id = player_id as usize;

        let Some(render_state) = self.players.get_mut(player_id) else {
            eprintln!("Unknown player {player_id}");
            return;
        };

        d.begin_texture(&mut render_state.framebuffer, || {
            ui(d);

            // SAFE: Raylib is initialized (`d` existence requirement).
            unsafe {
                rlDrawRenderBatchActive();
                rlEnableDepthTest();

                rlPushMatrix();
                rlMatrixMode(RL_PROJECTION as _);
                rlSetMatrixProjection(Matrix::from(Into::<ColumnMatrix4<f32>>::into(
                    Matrix4::new_perspective(
                        CAMERA_ASPECT,
                        CAMERA_FOVY * (DEG2RAD as f32),
                        RL_CULL_DISTANCE_NEAR as _,
                        RL_CULL_DISTANCE_FAR as _,
                    ),
                )));
                rlMatrixMode(RL_MODELVIEW as _);
                rlSetMatrixModelview(Matrix::from(Into::<ColumnMatrix4<f32>>::into(
                    Self::get_player_modelview_matrix(&render_state.player),
                )));
            }

            world(d);

            // SAFE: A matrix is pushed.
            unsafe {
                rlPopMatrix();
            }
        });
    }

    pub fn draw(&self, d: &RefMut<RaylibDrawHandle>) {
        d.draw_texture(&self.players[0].framebuffer, 0, 0, Color::WHITE);
        d.draw_texture(&self.players[1].framebuffer, self.fb_width, 0, Color::WHITE);
    }
}
