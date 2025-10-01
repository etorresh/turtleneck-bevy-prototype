use avian3d::prelude::*;
use bevy::{audio::AudioPlugin, prelude::*, window::WindowResolution};
mod components;
mod plugins;

use plugins::{
    camera::CameraPlugin, player::PlayerPlugin, shooting::ShootingPlugin, world::WorldPlugin,
};
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1920, 1080)
                            .with_scale_factor_override(1.0),
                        position: WindowPosition::Centered(MonitorSelection::Index(1)),
                        ..default()
                    }),
                    ..default()
                })
                .disable::<AudioPlugin>(),
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            PlayerPlugin,
            CameraPlugin,
            WorldPlugin,
            ShootingPlugin,
            // Inspector
            // EguiPlugin::default(),
            // WorldInspectorPlugin::new(),
        ))
        .run();
}
