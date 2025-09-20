use crate::components::camera::CameraFocus;
use crate::components::gamelayer::GameLayer;
use crate::components::player::{Player, PlayerSet};
use avian3d::{math::Vector, prelude::*};
use bevy::prelude::*;
use core::f32;

pub struct PlayerPlugin;

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
struct Speed(f32);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player.in_set(PlayerSet::Movement));
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneRoot(asset_server.load("turtle/Turtle.gltf#Scene0")),
        Transform::from_xyz(0.0, 0., 0.).with_scale(Vec3::splat(0.25)),
        Player,
        Collider::capsule(0.25, 1.),
        RigidBody::Kinematic,
        Speed(3.0),
        CameraFocus,
        Name::new("Player"),
    ));
}

fn move_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Speed, &Collider, Entity), With<Player>>,
    spatial_query: SpatialQuery,
    mut physics_time: ResMut<Time<Physics>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (mut player_transform, player_speed, player_collider, player_entity) =
        player_query.single_mut().unwrap();

    let original_y = player_transform.translation.y;

    // rotate to face mouse
    if let Some(cursor_pos) = windows.single().unwrap().cursor_position() {
        let (camera, camera_transform) = camera.single().unwrap();

        if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
            let player_height = player_transform.translation.y;
            let t = (player_height - ray.origin.y) / ray.direction.y;

            // If t is negative, the intersection is behind the camera
            if t >= 0.0 {
                let point = ray.get_point(t);
                let to_cursor = Vec2::new(point.x, point.z)
                    - Vec2::new(
                        player_transform.translation.x,
                        player_transform.translation.z,
                    );
                // Calculate angle in XY plane
                let angle = to_cursor.y.atan2(to_cursor.x);
                // Rotate only around Z axis
                player_transform.rotation =
                    Quat::from_rotation_y(-angle - std::f32::consts::FRAC_PI_2);
            }
        }
    }

    let direction = Vec3::new(
        (keys.pressed(KeyCode::KeyD) as i32 - keys.pressed(KeyCode::KeyA) as i32) as f32,
        0.,
        -((keys.pressed(KeyCode::KeyW) as i32 - keys.pressed(KeyCode::KeyS) as i32) as f32),
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
                    player_transform.translation.z,
                ),
                player_transform.rotation,
                Dir3::new_unchecked(movement_direction),
                &ShapeCastConfig::from_max_distance(remaining_distance),
                &SpatialQueryFilter::from_mask(GameLayer::Default)
                    .with_excluded_entities([player_entity]),
            );

            match shape_hit {
                Some(hit) => {
                    let safe_distance = (hit.distance - COLLISION_EPSILON).max(0.0);
                    let safe_movement = movement_direction * safe_distance;
                    if safe_distance > COLLISION_EPSILON {
                        player_transform.translation += safe_movement;
                        player_transform.translation.y = original_y;
                        remaining_distance -= safe_distance;
                    }
                    let horizontal_normal =
                        Vec3::new(hit.normal1.x, 0.0, hit.normal1.z).normalize();

                    let slide_vector =
                        raw_movement - horizontal_normal * raw_movement.dot(horizontal_normal);

                    let horizontal_slide = Vec3::new(slide_vector.x, 0.0, slide_vector.z);

                    movement_direction = match horizontal_slide.try_normalize() {
                        Some(dir) => dir,
                        None => break,
                    };
                }
                None => {
                    player_transform.translation += raw_movement;
                    break;
                }
            }
        }
    }

    // debugging keybinds
    {
        if keys.just_released(KeyCode::KeyH) {
            if physics_time.is_paused() {
                physics_time.unpause();
            } else {
                physics_time.pause();
            }
        }
    }
}
