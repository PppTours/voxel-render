use bevy_ecs::{component::Component, system::Resource};
use raylib::{ffi::{mint::Vector3, Camera3D, Color, KeyboardKey}, prelude::RaylibHandle};

#[derive(Clone, Copy, Component)]
pub struct Position(pub Vector3<f32>);

#[derive(Clone, Copy, Component, Debug)]
pub struct Player {
    pub camera: Camera3D,
    pub id: u32,
}

#[derive(Clone, Copy, Component)]
pub struct Cube {
    pub color: Color,
    pub size: Vector3<f32>,
}

#[derive(Default, Debug)]
pub struct PlayerInputInfo {
    pub view_up: bool,
    pub view_down: bool,
    pub view_left: bool,
    pub view_right: bool,

    pub fire_pressed: bool,
    pub fire_down: bool,

    pub move_forward: bool,
    pub move_backward: bool,
    pub move_left: bool,
    pub move_right: bool,

    pub jump_pressed: bool,
    pub jump_down: bool,
}

#[derive(Resource, Default, Debug)]
pub struct GameInput {
    pub player: [PlayerInputInfo; 2],
}

impl GameInput {
    pub fn update_inputs(&mut self, rl: &RaylibHandle) {
        // https://github.com/PppTours/launcherBorne#contr%C3%B4les
        // QWERTY!
        self.player[0] = PlayerInputInfo {
            view_up: rl.is_key_down(KeyboardKey::KEY_UP),
            view_down: rl.is_key_down(KeyboardKey::KEY_DOWN),
            view_left: rl.is_key_down(KeyboardKey::KEY_LEFT),
            view_right: rl.is_key_down(KeyboardKey::KEY_RIGHT),
            fire_pressed: rl.is_key_pressed(KeyboardKey::KEY_R),
            fire_down: rl.is_key_down(KeyboardKey::KEY_R),
            move_forward: rl.is_key_down(KeyboardKey::KEY_T),
            move_backward: rl.is_key_down(KeyboardKey::KEY_G),
            move_left: rl.is_key_down(KeyboardKey::KEY_F),
            move_right: rl.is_key_down(KeyboardKey::KEY_H),
            jump_pressed: rl.is_key_pressed(KeyboardKey::KEY_Y),
            jump_down: rl.is_key_down(KeyboardKey::KEY_Y),
        };

        self.player[1] = PlayerInputInfo {
            view_up: rl.is_key_down(KeyboardKey::KEY_W),
            view_down: rl.is_key_down(KeyboardKey::KEY_S),
            view_left: rl.is_key_down(KeyboardKey::KEY_A),
            view_right: rl.is_key_down(KeyboardKey::KEY_D),
            fire_pressed: rl.is_key_pressed(KeyboardKey::KEY_U),
            fire_down: rl.is_key_down(KeyboardKey::KEY_U),
            move_forward: rl.is_key_down(KeyboardKey::KEY_I),
            move_backward: rl.is_key_down(KeyboardKey::KEY_K),
            move_left: rl.is_key_down(KeyboardKey::KEY_J),
            move_right: rl.is_key_down(KeyboardKey::KEY_L),
            jump_pressed: rl.is_key_pressed(KeyboardKey::KEY_O),
            jump_down: rl.is_key_down(KeyboardKey::KEY_O),
        };

        println!("{self:#?}");
    }
}