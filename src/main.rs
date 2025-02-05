use avian2d::prelude::*;
use bevy::prelude::*;
mod components;
mod plugins;

use plugins::{camera::CameraPlugin, player::PlayerPlugin, world::WorldPlugin, shooting::ShootingPlugin};
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PlayerPlugin,
            CameraPlugin,
            WorldPlugin,
            ShootingPlugin
        ))
        .insert_resource(Gravity(Vec2::ZERO))
        .run();
}
