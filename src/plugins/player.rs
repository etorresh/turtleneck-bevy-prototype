use core::f32;

use crate::components::camera::CameraFocus;
use avian2d::{math::Vector, prelude::*};
use bevy::{math::NormedVectorSpace, prelude::*};
pub struct PlayerPlugin;

// offset that can prevent floating value errors in collisions
const COLLISION_OFFSET: f32 = f32::EPSILON * 100.0;

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
        let frame_budget = player_speed.0 * time.delta_secs();
        let mut remaining_budget = frame_budget;
        let mut movement_direction = direction.normalize();
        let filter = SpatialQueryFilter::default().with_excluded_entities([player_entity]);

        let mut attempts = 3;
        while remaining_budget > 0.0 && attempts > 0 {
            
            attempts -= 1;
            let desired_movement = movement_direction * remaining_budget;
            let origin = Vector::new(
                player_transform.translation.x,
                player_transform.translation.y,
            );
            let target_dir = Dir2::new_unchecked(movement_direction);
            // since the physics are 2d, we only care about the z-axis rotation from the quaternion
            let (_, _, rotation) = player_transform.rotation.to_euler(EulerRot::XYZ);

            match spatial_query.cast_shape(
                player_collider,
                origin,
                rotation,
                target_dir,
                &ShapeCastConfig::from_max_distance(remaining_budget),
                &filter,
            ) {
                Some(hit) => {
                    let move_distance = hit.distance - COLLISION_OFFSET;
                    if move_distance == 0.0 {
                        break;
                    }
                    player_transform.translation +=
                        (movement_direction * move_distance).extend(0.0);
                    remaining_budget -= move_distance;

                    let (body, mut velocity) = cast_query
                        .get_mut(hit.entity)
                        .expect("Missing Rigidbody component");
                    match body {
                        RigidBody::Dynamic => {
                            let push_force = target_dir * player_speed.0 * 2.0 * time.delta_secs();
                            velocity.x += push_force.x;
                            velocity.y += push_force.y;
                            break;
                        }
                        _ => {
                            // only try to slide if moving diagonally
                            if movement_direction.x == 0.0 || movement_direction.y == 0.0 {
                                break;
                            }
                            let slide_vector = desired_movement
                                - hit.normal1 * desired_movement.dot(hit.normal1);
                            movement_direction = match slide_vector.try_normalize() {
                                Some(dir) => dir,
                                None => break,
                            };
                        }
                    }
                }
                None => {
                    player_transform.translation += desired_movement.extend(0.0);
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
