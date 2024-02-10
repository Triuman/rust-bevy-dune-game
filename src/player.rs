use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier2d::dynamics::{LockedAxes, RigidBody};
use bevy_rapier2d::geometry::{ActiveEvents, Collider, Friction, Restitution};
use bevy_rapier2d::pipeline::{CollisionEvent, ContactForceEvent};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct TouchingGround;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_camera.run_if(in_state(GameState::Playing)))
            .add_systems(Update, display_events.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    /* Create the bouncing ball. */
    commands
        // .spawn(SpriteBundle {
        //     texture: textures.bevy.clone(),
        //     transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
        //     ..Default::default()
        // })
        .spawn(Player)
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(10.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(LockedAxes::ROTATION_LOCKED_Z)
        .insert(Restitution::coefficient(0.0))
        .insert(Friction::new(0.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 200.0, 0.0)));
}

fn move_camera(
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    // Attempt to get the single player and camera transform
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            // Update the camera's position to match the player's position
            camera_transform.translation.x = player_transform.translation.x;
            // camera_transform.translation.y = player_transform.translation.y;
        }
    }
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
    players: Query<&Player>,
    mut commands: Commands,
) {
    for collision_event in collision_events.read() {
        println!("Received collision event: {:?}", collision_event);
        // deconstruct the collision event
        match collision_event {
            CollisionEvent::Started(entity1, entity2, flags) => {
                println!(
                    "Collision started between entities {:?} and {:?} with flags {:?}",
                    entity1, entity2, flags
                );
                if players.get(*entity1).is_ok() {
                    commands
                        .get_entity(*entity1)
                        .unwrap()
                        .insert(TouchingGround);
                } else if players.get(*entity2).is_ok() {
                    commands
                        .get_entity(*entity2)
                        .unwrap()
                        .insert(TouchingGround);
                }
            }
            CollisionEvent::Stopped(entity1, entity2, flags) => {
                println!(
                    "Collision stopped between entities {:?} and {:?} with flags {:?}",
                    entity1, entity2, flags
                );
                if players.get(*entity1).is_ok() {
                    commands
                        .get_entity(*entity1)
                        .unwrap()
                        .remove::<TouchingGround>();
                } else if players.get(*entity2).is_ok() {
                    commands
                        .get_entity(*entity2)
                        .unwrap()
                        .remove::<TouchingGround>();
                }
            }
        }
    }

    for contact_force_event in contact_force_events.read() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}
