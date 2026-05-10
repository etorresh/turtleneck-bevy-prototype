use crate::components::{camera::CameraFocus};
use bevy::{post_process::bloom::Bloom, prelude::*, render::view::Hdr};
use crate::components::gamestate::{ActivityState};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, focus_camera.run_if(in_state(ActivityState::Playing)));
    }
}

fn spawn_camera(mut commands: Commands) {
    // camera
    commands.spawn((
        Camera3d::default(),
        Hdr,
        Bloom::default(),
        Transform::from_xyz(0., 10., 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn focus_camera(
    mut camera_transform: Single<(&mut Transform), (With<Camera>, Without<CameraFocus>)>,
    subject_query: Query<&Transform, With<CameraFocus>>,
) {
    let mut focus_points = Vec::new();

    for transform in subject_query.iter() {
        focus_points.push(transform.translation);
    }

    if !focus_points.is_empty() {
        let avg_position = focus_points.iter().sum::<Vec3>() / focus_points.len() as f32;

        // Camera follows on X-Z plane, stays high on Y-axis
        let target_camera_pos = Vec3::new(
            avg_position.x,        // Follow X
            10.0,                  // Stay high on Y (was Z)
            avg_position.z + 10.0, // Follow Z with offset (was Y)
        );

        camera_transform.translation = camera_transform.translation.lerp(target_camera_pos, 0.1);

        // Zoom based on spread
        let max_distance = focus_points
            .iter()
            .map(|pos| pos.distance(avg_position))
            .fold(0.0, f32::max);

        camera_transform.translation.y = 10.0 + max_distance; // Adjust Y height for zoom
    }
}
