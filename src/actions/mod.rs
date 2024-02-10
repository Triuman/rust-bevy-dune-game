use bevy::prelude::*;
use bevy_rapier2d::dynamics::ExternalImpulse;

use crate::actions::game_control::GameControl;
use crate::player::{Player, TouchingGround};
use crate::GameState;

mod game_control;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_systems(
            Update,
            set_movement_actions.run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
}

pub fn set_movement_actions(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    player_air: Query<Entity, (With<Player>, Without<TouchingGround>)>,
    player_ground: Query<Entity, (With<Player>, With<TouchingGround>)>,
) {
    let is_pressed = GameControl::Space.pressed(&keyboard_input);

    // insert  ExternalImpulse to the player
    if is_pressed {
        for player_entity in player_air.iter() {
            commands.entity(player_entity).insert(ExternalImpulse {
                impulse: Vec2::new(0.0, -0.2),
                torque_impulse: 0.0,
            });
        }
        for player_entity in player_ground.iter() {
            commands.entity(player_entity).insert(ExternalImpulse {
                impulse: Vec2::new(0.3, 0.05),
                torque_impulse: 0.0,
            });
        }
    }
}
