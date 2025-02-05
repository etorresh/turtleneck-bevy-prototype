use avian2d::prelude::*;
use bevy::prelude::*;
use crate::components::player::Player;
use crate::components::gamelayer::GameLayer;

pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_shooting, move_bullets));
    }
}

fn handle_shooting(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    player_query: Query<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
                    let to_mouse = (Vec2::new(point.x, point.y) - 
                                  Vec2::new(player_transform.translation.x, 
                                          player_transform.translation.y)).normalize();
                    
                    // Spawn bullet with velocity in that direction
                    commands.spawn((
                        Mesh3d(meshes.add(Sphere::new(0.1))),
                        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
                        Transform::from_translation(player_transform.translation),
                        Collider::circle(0.1),
                        Bullet {
                            velocity: to_mouse * 10.0,
                            push_force: 5.0
                        },
                        CollisionLayers::new(GameLayer::PlayerBullet, GameLayer::Default),
                    ));
                }
            }
        }
    }
}

#[derive(Component)]
struct Bullet {
    velocity: Vec2,
    push_force: f32,
}

// Add system to move bullets
fn move_bullets(
    mut bullets: Query<(&mut Transform, &Bullet)>,
    time: Res<Time>,
) {
    for (mut transform, bullet) in &mut bullets {
        transform.translation += bullet.velocity.extend(0.0) * time.delta_secs();
    }
}