use crate::components::{enemy::Enemy, gamelayer::GameLayer, health::Health};
use avian3d::prelude::*;
use bevy::prelude::*;

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
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(12.0, 12.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Collider::cuboid(12., 0.0, 12.),
        RigidBody::Static,
        CollisionLayers::new(GameLayer::Floor, GameLayer::Default),
    ));
}

fn spawn_light(mut commands: Commands) {
    // Light above the scene
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(Color::srgb_u8(0, 255, 0))),
        Transform::from_xyz(2.5, 10., 0.0), // Y is now height
        Collider::cuboid(0.5, 0.5, 0.5),
        RigidBody::Dynamic,
        TransformInterpolation,
        LinearDamping(0.9),
        AngularDamping(0.9),
        Name::new("Dynamic Cube"),
    ));

    // Enemy cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 0, 0))),
        Transform::from_xyz(0.0, 0.25, 2.5), // Now at positive Z instead of Y
        Collider::cuboid(0.5, 0.5, 0.5),
        RigidBody::Dynamic,
        TransformInterpolation,
        LinearDamping(0.9),
        AngularDamping(0.9),
        Enemy,
        Health(2),
        Name::new("Enemy"),
    ));

    // Static cubes
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(-2.5, 0.25, 0.0),
        Collider::cuboid(0.5, 0.5, 0.5),
        RigidBody::Static,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(-3.0, 0.25, 0.0),
        Collider::cuboid(0.5, 0.5, 0.5),
        RigidBody::Static,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(-3.0, 0.25, 0.5),
        Collider::cuboid(0.5, 0.5, 0.5),
        RigidBody::Static,
    ));

    // Rotated cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(-3.0, 0.25, 3.0)
            .with_rotation(Quat::from_rotation_y(45_f32.to_radians())), // Rotate around Y (vertical) axis
        Collider::cuboid(0.5, 0.5, 0.5),
        RigidBody::Static,
    ));
}
