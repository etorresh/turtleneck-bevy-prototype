use avian2d::prelude::*;
use bevy::prelude::*;

const GRID_SIZE: f32 = 16.0; // Size of the world in units
const WALL_THICKNESS: f32 = 0.5;
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_floor, spawn_light, spawn_cube));
    }
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Existing floor
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(GRID_SIZE, GRID_SIZE, 1.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, -0.5),
    ));

    // Spawn boundary colliders
    // Top wall
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(GRID_SIZE, WALL_THICKNESS),
        Transform::from_xyz(0.0, GRID_SIZE/2.0 + WALL_THICKNESS / 2.0, 0.0),
    ));

    // Bottom wall
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(GRID_SIZE, WALL_THICKNESS),
        Transform::from_xyz(0.0, -GRID_SIZE/2.0 - WALL_THICKNESS / 2.0, 0.0),
    ));

    // Left wall
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(WALL_THICKNESS, GRID_SIZE),
        Transform::from_xyz(-GRID_SIZE/2.0 - WALL_THICKNESS / 2.0, 0.0, 0.0),
    ));

    // Right wall
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(WALL_THICKNESS, GRID_SIZE),
        Transform::from_xyz(GRID_SIZE/2.0 + WALL_THICKNESS / 2.0, 0.0, 0.0),
    ));
}

fn spawn_light(mut commands: Commands) {
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            range: GRID_SIZE,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0,8.0).looking_at(Vec3::ZERO, Vec3::Z),
    ));
}

fn spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube dynamic
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(2.5, 0.0, 0.25),
        Collider::rectangle(0.5, 0.5),
        RigidBody::Dynamic,
        TransformInterpolation,
        LinearDamping(0.9),
        AngularDamping(0.9)
    ));

    // cube static
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(-2.5, 0.0, 0.25),
        Collider::rectangle(0.5, 0.5),
        RigidBody::Static,
    ));
    // cube static
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(-3.0, 0.0, 0.25),
        Collider::rectangle(0.5, 0.5),
        RigidBody::Static,
    ));
    // cube static
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(-3.0, 0.5, 0.25),
        Collider::rectangle(0.5, 0.5),
        RigidBody::Static,
    ));

    // cube static
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(-3.0, 3.0, 0.25)
            .with_rotation(Quat::from_rotation_z(45_f32.to_radians())),
        Collider::rectangle(0.5, 0.5),
        RigidBody::Static,
    ));
}
