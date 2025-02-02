use core::f32;
use std::{alloc::System, time::Duration};

use avian2d::{math::Vector, prelude::*};
use bevy::{ecs::system::SystemState, prelude::*};
use crate::components::camera::CameraFocus;
pub struct PlayerPlugin;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.0, 0.25),
        Player,
        Collider::rectangle(0.5, 0.5),
        RigidBody::Kinematic,
        Speed(3.0),
        CameraFocus,
    ));
}

fn move_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Speed, &Collider, Entity), With<Player>>,
    mut cast_query: Query<(&RigidBody, &mut LinearVelocity)>,
    spatial_query: SpatialQuery,
    mut physics_time: ResMut<Time<Physics>>,
) {
    let (mut player_transform, player_speed, player_collider, player_entity) = query.single_mut();

    let mut direction = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    
    if direction.length_squared() > 0.0 {
        let original_movement = direction * player_speed.0 * time.delta_secs();
        let mut remaining_movement = original_movement;

        let filter = SpatialQueryFilter::default().with_excluded_entities([player_entity]);
        while remaining_movement.length_squared() > f32::EPSILON {
            let origin = Vector::new(player_transform.translation.x, player_transform.translation.y);
            let target_dir = Dir2::new_unchecked(remaining_movement.normalize());
            // since the physics are 2d, we only care about the z-axis rotation from the quaternion
            let (_, _, rotation) = player_transform.rotation.to_euler(EulerRot::XYZ);
            let config = ShapeCastConfig::from_max_distance(remaining_movement.length());

            match spatial_query.cast_shape( player_collider, origin, rotation, target_dir, &config, &filter) {
                Some(hit) => {
                    let (body, mut velocity) = cast_query.get_mut(hit.entity).expect("Missing Rigidbody component");

                    let move_distance = (hit.distance - f32::EPSILON * 100.0).max(0.0);
                    let move_vec = target_dir * move_distance;

                    player_transform.translation += move_vec.extend(0.0);
                    remaining_movement -= move_vec;

                    match body {
                        RigidBody::Dynamic => {
                            let push_force = target_dir * player_speed.0 * 2.0 * time.delta_secs();
                            velocity.x += push_force.x;
                            velocity.y += push_force.y;
                            break
                        },
                        _ => {
                            let original_speed = remaining_movement.length();
                            let new_direction = remaining_movement - hit.normal1 * remaining_movement.dot(hit.normal1); // here the movement that goes into the obstacle is lost
                            remaining_movement = new_direction * original_speed; // multiply by original speed to recover lost speed
                        }
                    }

                },
                None => {
                    player_transform.translation += remaining_movement.extend(0.0);
                    break;
                }
            }
        }
    }


    // debugging keybinds
    {
        if keys.pressed(KeyCode::KeyF) {
            player_transform.translation.z += 1.0 * player_speed.0 * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyG) {
            player_transform.translation.z -= 1.0 * player_speed.0 * time.delta_secs();
        }
        player_transform.translation.z = player_transform.translation.z.max(0.25); 

        if keys.just_released(KeyCode::KeyH) {
            if physics_time.is_paused() {
                physics_time.unpause();
            } else {
                physics_time.pause();
            }
        }
    }
}