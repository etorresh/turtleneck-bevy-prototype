use avian2d::{math::Vector, prelude::*};
use bevy::prelude::*;
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
        CameraFocus
    ));
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Speed, &Collider, Entity), With<Player>>,
    spatial_query: SpatialQuery,
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
        direction = direction.normalize();
        let target = Dir2::new_unchecked(direction);
        let origin = Vector::new(player_transform.translation.x, player_transform.translation.y);
        // since the physics are 2d, we only care about the z-axis rotation from the quaternion
        let (_, _, rotation) = player_transform.rotation.to_euler(EulerRot::XYZ);
        let config = ShapeCastConfig::from_max_distance(player_speed.0 * time.delta_secs());
        let filter = SpatialQueryFilter::default().with_excluded_entities([player_entity]);

        if let Some(first_hit) = spatial_query.cast_shape( player_collider, origin, rotation, target, &config, &filter) {
            println!("First hit: {:?}", first_hit);
            let movement = direction * (first_hit.distance + 0.01);
            player_transform.translation.x += movement.x;
            player_transform.translation.y += movement.y;

        } else {
            let mut dynamic_speed = player_speed.0;
            let movement = direction * dynamic_speed * time.delta_secs();
    
            player_transform.translation.x += movement.x;
            player_transform.translation.y += movement.y;
        }
    }



    // delete this block later, it's just for fun and to showcase the 2d collisions with 3d models
    {
        if keys.pressed(KeyCode::KeyF) {
            player_transform.translation.z += 1.0 * player_speed.0 * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyG) {
            player_transform.translation.z -= 1.0 * player_speed.0 * time.delta_secs();
        }
        player_transform.translation.z = player_transform.translation.z.max(0.25); 
    }
}
