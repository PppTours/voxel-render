use std::cell::RefMut;

use bevy_ecs::system::Resource;
use raylib::{core::texture::RenderTexture2D, ffi::Camera3D, prelude::*};

use crate::{Cube, Position};

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
                d.draw_cube_v(position.0, cube.size, cube.color)
            }
            RenderCommand::CubeWithWires { position, cube } => {
                d.draw_cube_v(position.0, cube.size, cube.color);
                d.draw_cube_wires(
                    position.0,
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
    pub camera: Camera3D,
    pub framebuffer: RenderTexture2D<'bind, 'a>,
}

pub struct RenderState<'bind, 'a> {
    pub players: [PlayerRenderState<'bind, 'a>; 2],
}

impl<'bind, 'a> RenderState<'bind, 'a> {
    pub fn new(rl: &'bind RaylibHandle, t: &RaylibThread, cameras: [Camera3D; 2]) -> Self {
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
