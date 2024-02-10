use bevy::prelude::*;
use bevy_rapier2d::geometry::Collider;
use std::f32::consts::PI;

use crate::GameState;

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_ground);
    }
}

fn spawn_ground(mut commands: Commands) {
    let scale = 20000.0; // width of the ground
    let max_height = 30.0; // max height of the ground
    let heights: Vec<f32> = (0..(scale as f32).round() as i32)
        .map(|i| ((i as f32) / 150.0 * PI * 1.0).sin())
        .collect();

    // The transform doesn't seems to affect the ground's position
    commands
        .spawn(Transform::from_xyz(scale * 100., -max_height, 0.0))
        .insert(Collider::heightfield(heights, Vec2::new(scale, max_height)));
}
