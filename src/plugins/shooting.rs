use crate::components::enemy::Enemy;
use crate::components::gamelayer::GameLayer;
use crate::components::health::Health;
use crate::components::player::Player;
use avian2d::prelude::*;
use bevy::prelude::*;

pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GunConfig>()
            .add_systems(
                Update,
                (handle_shooting, move_bullets, handle_bullet_collisions).chain(),
            )
            .register_type::<GunConfig>();
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct GunConfig {
    starting_speed: f32,
    acceleration: f32,
    push_force: f32,
}

impl Default for GunConfig {
    fn default() -> Self {
        Self {
            starting_speed: 10.0,
            acceleration: 6.0,
            push_force: 2.0,
        }
    }
}

#[derive(Component)]
struct Bullet {
    direction: Dir2,
    current_speed: f32,
}

fn handle_shooting(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    player_query: Query<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    gun_config: Res<GunConfig>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Some(cursor_pos) = windows.single().cursor_position() {
            let (camera, camera_transform) = camera.single();

            if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
                let player_transform = player_query.single();
                // Find intersection with plane at player's Z height
                let t = (player_transform.translation.z - ray.origin.z) / ray.direction.z;

                if t >= 0.0 {
                    let point = ray.get_point(t);
                    // Get direction from player to mouse point
                    let to_mouse = (Vec2::new(point.x, point.y)
                        - Vec2::new(
                            player_transform.translation.x,
                            player_transform.translation.y,
                        ))
                    .normalize();

                    // Spawn bullet with velocity in that direction
                    commands.spawn((
                        Mesh3d(meshes.add(Sphere::new(0.1))),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: Color::srgb(1.0, 0.0, 0.0),
                            emissive: LinearRgba::new(10000., 0., 0., 0.),
                            ..default()
                        })),
                        Transform::from_translation(player_transform.translation),
                        Collider::circle(0.1),
                        Bullet {
                            direction: Dir2::new_unchecked(to_mouse),
                            current_speed: gun_config.starting_speed,
                        },
                        CollisionLayers::new(GameLayer::PlayerBullet, GameLayer::Default),
                    ));
                }
            }
        }
    }
}

fn move_bullets(
    time: Res<Time>,
    mut bullets: Query<(&mut Transform, &mut Bullet)>,
    gun_config: Res<GunConfig>,
) {
    for (mut transform, mut bullet) in &mut bullets {
        bullet.current_speed *= 1.0 + time.delta_secs() * gun_config.acceleration;
        let movement = bullet.direction * bullet.current_speed * time.delta_secs();
        transform.translation += movement.extend(0.0);
    }
}

fn handle_bullet_collisions(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Transform, &mut Bullet, &Collider)>,
    mut rigid_bodies: Query<(&RigidBody, &mut LinearVelocity)>,
    mut query_enemy: Query<&mut Health, With<Enemy>>,
    spatial_query: SpatialQuery,
    player_query: Query<Entity, With<Player>>,
    gun_config: Res<GunConfig>,
) {
    let player_entity = player_query.single();

    for (bullet_entity, transform, bullet, collider) in &mut bullets {
        // Then check for collision at new position
        if let Some(hit) = spatial_query.cast_shape(
            collider,
            Vec2::new(transform.translation.x, transform.translation.y),
            0.0,
            bullet.direction,
            &ShapeCastConfig::from_max_distance(0.001), // Small distance check
            &SpatialQueryFilter::from_mask(GameLayer::Default)
                .with_excluded_entities([player_entity]),
        ) {
            if let Ok((body, mut velocity)) = rigid_bodies.get_mut(hit.entity) {
                if matches!(body, RigidBody::Dynamic) {
                    velocity.0 += bullet.direction * gun_config.push_force;
                }
            }
            if let Ok(mut health) = query_enemy.get_mut(hit.entity) {
                if let Some(new_health) = health.0.checked_sub(1) {
                    health.0 = new_health;
                } else {
                    commands.entity(hit.entity).despawn();
                }
            }
            commands.entity(bullet_entity).despawn();
        }
    }
}
