use crate::components::camera::CameraFocus;
use avian2d::{math::Vector, prelude::*};
use bevy::{prelude::*, state::commands};
use core::f32;
use std::{os::linux::raw, time::Duration};
pub struct PlayerPlugin;

/* 
TO DO:
- dynamic rigidbodies with linear_damping feel jittery to push around,
that's because we push them, and then catch up to them and push them again
compared to having no linear_damping were they just move away from the player
matching its speed. the solution is to add a system that keeps track of the objects
being pushed and sets their linear damping to zero, and then once the player is no longer
touching them it sets it back to their original value. It might be important for the push
force to be related to the players speed, that way the system can be integrated with
a weight system that also makes certain objects harder to push. Actually this is not a great solution,
it means that the push_force should always be strong enough to get ahead of the player, I will keep
push force at 2 so it is always ahead of the player and implement a simple system that keeps track
of the last collision, if it is different than the current one then return LinearDamping and disable it
on the current one. This works for a very specific push_force, so a more general solution might be
to force a physics step after every dynamic collision (this is the only way I can think of to stop the jitter)
/* 
Objects with higher weight/friction may cause jittery pushing because:
1. Player pushes and adds force
2. Player moves again before next FixedUpdate
3. Creates catch-up loop until FixedUpdate moves the object

Two solutions:
- Current: Zero friction during push + high enough force (2.0) to prevent catch-up
- Future: Force physics step after push to ensure object moves before next player movement
*/
*/

// offset to avoid floating-point precision errors in collision detection
// a value of 80_000.0 * f32::EPSILON seems to work even for sharp corners (around 0.0095)
// but it might fail at high frame rates and get the player stuck
// an alternative approach is detecting static rigid bodies and recalculating the path,
// but this would prevent getting as close to static objects since you're limited by
// your own speed (if you're able to travel further than the distance to the static object then you won't move at all)
const COLLISION_EPSILON: f32 = f32::EPSILON * 80_000.0;
// 2 iterations are enough to resolve corner cases: 
// 1st handles the first wall, 2nd resolves the second wall (if in a corner)
// A 3rd iteration isn't needed, as movement becomes negligible (this might change if the player speed changes)
// I lean towards keeping it at 2 because values greater than 2 jitter when colliding with sharp colliders
const MAX_MOVEMENTS: u8 = 2;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct PushForce(f32);
#[derive(Component)]
struct PushForcePause(f32);

#[derive(Resource)]
struct ForcePhysicsStep(bool);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player)
            .add_systems(Update, step_physics_if_needed.run_if(force_physics).after(move_player))
            .insert_resource(ForcePhysicsStep(false));
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
        PushForce(2.0),
        PushForcePause(4.0),
    ));
}

fn move_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Speed, &Collider, Entity, &mut PushForce, &mut PushForcePause), With<Player>>,
    mut collision_target_query: Query<(&RigidBody, &mut LinearVelocity)>,
    spatial_query: SpatialQuery,
    mut physics_time: ResMut<Time<Physics>>,
) {
    let (mut player_transform, player_speed, player_collider, player_entity, mut push_force, mut push_force_paused) = player_query.single_mut();

    let direction = Vec2::new(
        (keys.pressed(KeyCode::KeyD) as i32 - keys.pressed(KeyCode::KeyA) as i32) as f32,
        (keys.pressed(KeyCode::KeyW) as i32 - keys.pressed(KeyCode::KeyS) as i32) as f32,
    );

    if direction.length_squared() > 0.0 {
        let mut remaining_distance = player_speed.0 * time.delta_secs();
        let mut movement_direction = direction.normalize();
        for _ in 0..MAX_MOVEMENTS {
            // 0.0 instead of COLLISION_EPSILON to allow movement towards dynamic rigidbodies
            if remaining_distance <= 0.0 {
                break;
            }
            let raw_movement = movement_direction * remaining_distance;

            let shape_hit = spatial_query.cast_shape(
                player_collider,
                Vector::new(
                    player_transform.translation.x,
                    player_transform.translation.y,
                ),
                player_transform.rotation.to_euler(EulerRot::XYZ).2,
                Dir2::new_unchecked(movement_direction),
                &ShapeCastConfig::from_max_distance(remaining_distance),
                &SpatialQueryFilter::default().with_excluded_entities([player_entity]),
            );

            match shape_hit {
                Some(hit) => {
                    let (body, mut velocity) = collision_target_query
                        .get_mut(hit.entity)
                        .expect("Missing Rigidbody component");
                    let safe_distance = (hit.distance - COLLISION_EPSILON).max(0.0);
                    let safe_movement = movement_direction * safe_distance;
                    match body {
                        RigidBody::Dynamic => {
                            player_transform.translation += safe_movement.extend(0.0);
                            velocity.0 += movement_direction * player_speed.0 * push_force.0 * time.delta_secs();
                            break;
                        }
                        _ => {
                            if safe_distance > COLLISION_EPSILON {
                                player_transform.translation += (safe_movement).extend(0.0);
                                remaining_distance -= safe_distance;
                            }
                            let slide_vector = raw_movement - hit.normal1 * raw_movement.dot(hit.normal1);
                            movement_direction = match slide_vector.try_normalize() {
                                Some(dir) => dir,
                                None => {
                                    break},
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
            std::mem::swap(&mut push_force.0, &mut push_force_paused.0);
            if physics_time.is_paused() {
                physics_time.unpause();
            } else {
                physics_time.pause();
            }
        }
    }
}

pub fn step_physics_if_needed(world: &mut World) {
    world.resource_mut::<Time<Physics>>().advance_by(Duration::from_secs_f64(1.0/64.0));
    world.run_schedule(PhysicsSchedule);
    world.resource_mut::<ForcePhysicsStep>().0 = false;
}

fn force_physics(should_step_phyisics: Res<ForcePhysicsStep>) -> bool {
    should_step_phyisics.0
}
