use core::f32;

use crate::components::camera::CameraFocus;
use avian2d::{math::Vector, prelude::*};
use bevy::prelude::*;
pub struct PlayerPlugin;

// offset to avoid floating-point precision errors in collision detection
// a value of 80_000.0 * f32::EPSILON seems to work even for sharp corners (around 0.0095)
// but it might fail at high frame rates and get the player stuck 
// an alternative approach is detecting static rigid bodies and recalculating the path,
// but this would prevent getting as close to static objects since you're limited by
// your own speed (if you're able to travel further than the distance to the static object then you won't move at all)
const COLLISION_EPSILON: f32 = f32::EPSILON * 80_000.0;
const MAX_MOVEMENTS: u8 = 2;

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

    let direction = Vec2::new(
        (keys.pressed(KeyCode::KeyD) as i32 - keys.pressed(KeyCode::KeyA) as i32) as f32,
        (keys.pressed(KeyCode::KeyW) as i32 - keys.pressed(KeyCode::KeyS) as i32) as f32,
    );

    if direction.length_squared() > 0.0 {
        let mut remaining_distance = player_speed.0 * time.delta_secs();
        let mut movement_direction = direction.normalize();
        for _ in 0..MAX_MOVEMENTS {
            // 0.0 instead of COLLISION_EPSILON to allow movement right next to dynamic rigidbodies
            if remaining_distance <= 0.0 {
                break;
            }
            let raw_movement = movement_direction * remaining_distance;

            let shape_hit = spatial_query.cast_shape (
                player_collider,
                Vector::new(player_transform.translation.x, player_transform.translation.y,),
                player_transform.rotation.to_euler(EulerRot::XYZ).2,
                Dir2::new_unchecked(movement_direction),
                &ShapeCastConfig::from_max_distance(remaining_distance),
                &SpatialQueryFilter::default().with_excluded_entities([player_entity]),
            );

            match shape_hit {
                Some(hit) => {
                    let (body, mut velocity) = cast_query
                        .get_mut(hit.entity)
                        .expect("Missing Rigidbody component");
                    let safe_distance = (hit.distance - COLLISION_EPSILON).max(0.0);
                    let safe_movement = movement_direction * safe_distance;
                    match body {
                        RigidBody::Dynamic => {
                            // using the hit.distance * movement_direction and slightly clipping into dynamic objects looks better BUT
                            // it blocks movement if physics are paused (which will be a valid powerup for the player) so let's
                            // use the safe_movement instead
                            player_transform.translation += safe_movement.extend(0.0);
                            let push_force = movement_direction * player_speed.0 * 2.0 * time.delta_secs();
                            velocity.0 += push_force;
                            break;
                        }
                        _ => {
                            if safe_distance > COLLISION_EPSILON {
                                player_transform.translation += (safe_movement).extend(0.0);
                                remaining_distance -= safe_distance;
                            }
                            let slide_vector = raw_movement
                                - hit.normal1 * raw_movement.dot(hit.normal1);
                            movement_direction = match slide_vector.try_normalize() {
                                Some(dir) => dir,
                                None => break,
                            };
                        }
                    }
                }
                None => {
                    player_transform.translation += raw_movement.extend(0.0);
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
