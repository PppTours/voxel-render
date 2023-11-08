use std::cell::RefMut;

use bevy_ecs::system::Resource;
use nalgebra::{Matrix4, Point3, Quaternion, Vector3};
use raylib::{
    core::texture::RenderTexture2D,
    ffi::{
        rlDrawRenderBatchActive, rlLoadIdentity, rlMatrixMode, rlOrtho, rlPopMatrix, rlPushMatrix,
        rlRotatef, rlSetMatrixModelview, rlSetMatrixProjection, rlTranslatef, RL_CULL_DISTANCE_FAR,
        RL_CULL_DISTANCE_NEAR, RL_MODELVIEW, RL_PROJECTION,
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
        println!("{}", player.rotation.angle());
        Matrix4::new_rotation_wrt_point(
            Vector3::y() * player.rotation.angle() * 0.0f32,
            Point3::new(player.position.x, player.height, player.position.y),
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

                rlMatrixMode(RL_PROJECTION as _);
                rlPushMatrix();
                rlSetMatrixProjection(
                    Matrix4::new_perspective(
                        8f32 / 9f32, // hardcoded
                        70f32,
                        RL_CULL_DISTANCE_NEAR as _,
                        RL_CULL_DISTANCE_FAR as _,
                    )
                    .transpose()
                    .into(),
                );
                rlMatrixMode(RL_MODELVIEW as _);
                rlLoadIdentity();
                rlRotatef(render_state.player.rotation.angle(), 0.0, 1.0, 0.0);
                rlTranslatef(
                    render_state.player.position.x,
                    render_state.player.height,
                    render_state.player.position.y,
                );
                //rlSetMatrixModelview(
                //    Self::get_player_modelview_matrix(&render_state.player).into(),
                //);
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
