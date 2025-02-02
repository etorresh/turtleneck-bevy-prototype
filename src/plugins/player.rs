use core::f32;
use std::{alloc::System, time::Duration};

use avian2d::{math::Vector, prelude::*};
use bevy::{ecs::system::SystemState, prelude::*};
use crate::components::camera::CameraFocus;
pub struct PlayerPlugin;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player)
            // .add_systems(Update, step_physics_if_needed.run_if(force_physics).after(move_player))
            .insert_resource(ShouldStepPhysics(false));
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
        CameraFocus,
    ));
}

fn move_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Speed, &Collider, Entity), With<Player>>,
    mut cast_query: Query<(&RigidBody, &mut LinearVelocity)>,
    spatial_query: SpatialQuery,
    mut physics_time: ResMut<Time<Physics>>,
    mut force_physics: ResMut<ShouldStepPhysics>
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
            let (first_hit_rigidbody, mut first_hit_linear_velocity) = cast_query.get_mut(first_hit.entity).expect("Entity with GameLayer(Rigidbody) did not have Rigidbody component");
            
            let safe_offset = f32::EPSILON * 100.0;
            let adjusted_distance = first_hit.distance - safe_offset;
            let movement = direction * adjusted_distance;

            player_transform.translation.x += movement.x;
            player_transform.translation.y += movement.y;

            if first_hit_rigidbody.is_dynamic() {
                let push_force = direction * player_speed.0 * 2.0 * time.delta_secs();
                first_hit_linear_velocity.x += push_force.x;
                first_hit_linear_velocity.y += push_force.y;
                println!("setting to true");
                force_physics.0 = true;
            }
        } else {
            let movement = direction * player_speed.0 * time.delta_secs();

            player_transform.translation.x += movement.x;
            player_transform.translation.y += movement.y;
        }
    }



    // debugging keybinds
    {
        if keys.pressed(KeyCode::KeyF) {
            player_transform.translation.z += 1.0 * player_speed.0 * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyG) {
            player_transform.translation.z -= 1.0 * player_speed.0 * time.delta_secs();
        }
        player_transform.translation.z = player_transform.translation.z.max(0.25); 

        if keys.just_released(KeyCode::KeyH) {
            if physics_time.is_paused() {
                physics_time.unpause();
            } else {
                physics_time.pause();
            }
        }
    }
}


// physics setup to force steps
#[derive(Resource, Default)]
pub struct ShouldStepPhysics(pub bool);

fn force_physics(should_step_phyisics: Res<ShouldStepPhysics>) -> bool {
    should_step_phyisics.0
}

pub fn step_physics_if_needed(
    world: &mut World,
) {
    println!("exclusive");
    // for _ in 0..1 {
    //     world.resource_mut::<Time<Physics>>().advance_by(Duration::from_secs_f64(1.0/64.0));
    //     world.run_schedule(PhysicsSchedule);
    // }
    world.resource_mut::<ShouldStepPhysics>().0 = false;
    println!("leaving exclusive");
}