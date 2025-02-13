use avian2d::prelude::*;
use bevy::{prelude::*, window::WindowResolution};
mod components;
mod plugins;

use plugins::{
    camera::CameraPlugin, player::PlayerPlugin, shooting::ShootingPlugin, world::WorldPlugin,
};

use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(1600., 900.).with_scale_factor_override(1.0),
                    position: WindowPosition::Centered(MonitorSelection::Index(1)),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            PlayerPlugin,
            CameraPlugin,
            WorldPlugin,
            ShootingPlugin,
            // Inspector
            WorldInspectorPlugin::new(),
        ))
        .insert_resource(Gravity(Vec2::ZERO))
        .run();
}
