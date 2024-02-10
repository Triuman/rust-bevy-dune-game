use bevy::prelude::{Input, KeyCode, Res};

pub enum GameControl {
    Space,
}

impl GameControl {
    pub fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Space => keyboard_input.pressed(KeyCode::Space),
        }
    }
}
