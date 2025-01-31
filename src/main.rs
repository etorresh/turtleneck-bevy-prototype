use bevy::{math::VectorSpace, prelude::*};

#[derive(Component)]
struct Player;

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>
) {
    let mut player_transform = query.single_mut();
    let speed = 3.0;

    let direction = {
        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyD) {direction.x += 1.0;}
        if keys.pressed(KeyCode::KeyA) {direction.x -= 1.0;} 
        if keys.pressed(KeyCode::KeyW) {direction.z -= 1.0;} 
        if keys.pressed(KeyCode::KeyS) {direction.z += 1.0;}
        
        if direction.length_squared() > 0.0 {direction = direction.normalize();}

        direction
    };

    player_transform.translation += direction * speed * time.delta_secs();
}
 
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, move_player)
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    )));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.25, 0.0),
        Player
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 10.0, 4.0),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
 