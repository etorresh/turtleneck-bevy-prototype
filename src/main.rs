use avian2d::prelude::*;
use bevy::prelude::*;

mod components;
mod plugins;

use plugins::{camera::CameraPlugin, player::PlayerPlugin, world::WorldPlugin};
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PlayerPlugin,
            CameraPlugin,
            WorldPlugin,
        ))
        .insert_resource(Gravity(Vec2::ZERO))
        .run();
}
