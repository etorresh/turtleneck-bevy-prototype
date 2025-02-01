use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform {
            translation: Vec3::new(0.0, 0.0, 10.0),
            // Apply a -90° rotation around the Z-axis
            rotation: Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2),
            ..Default::default()
        }
        .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
