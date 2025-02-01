use bevy::prelude::*;
use crate::components::camera::CameraFocus;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.
        add_systems(Startup, spawn_camera)
        .add_systems(Update, focus_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform {
            translation: Vec3::new(0.0, -5.0, 10.0),
            // Apply a -90° rotation around the Z-axis
            rotation: Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2),
            ..Default::default()
        }
        .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn focus_camera(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<CameraFocus>)>,
    subject_query: Query<&Transform, With<CameraFocus>>
) {
    let mut camera_transform = camera_query.single_mut();

    let mut focus_points = Vec::new();
    for transform in subject_query.iter() {
        focus_points.push(transform.translation);
    } 

    if !focus_points.is_empty() {
        let avg_position = focus_points.iter().sum::<Vec3>() / focus_points.len() as f32;
        
        // adjust camera position smoothly
        camera_transform.translation = camera_transform.translation.lerp(avg_position + Vec3::new(0.0, -5.0, 10.0), 0.1);
        
        // Optionally zoom out if there are many focus points (bullets, player, enemies)
        let max_distance = focus_points.iter()
            .map(|pos| pos.distance(avg_position))
            .fold(0.0, f32::max);
        
        camera_transform.translation.z = 10.0 + max_distance; // Adjust zoom based on spread
    }
}
