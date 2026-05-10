use avian3d::prelude::*;
use bevy::{audio::AudioPlugin, prelude::*, window::WindowResolution};
mod components;
mod plugins;

use plugins::{
    camera::CameraPlugin, player::PlayerPlugin, shooting::ShootingPlugin, level::WorldPlugin, world_switching::WorldSwitchingPlugin, input::InputPlugin
};

use crate::components::gamestate::{ActivityState, LocationState};
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
            ))
        .init_state::<ActivityState>()
        .init_state::<LocationState>()
        .add_plugins((
            PhysicsPlugins::default(),
            //PhysicsDebugPlugin::default(),
            InputPlugin,
            PlayerPlugin,
            CameraPlugin,
            WorldPlugin,
            ShootingPlugin,
            WorldSwitchingPlugin,
            // Inspector
            // EguiPlugin::default(),
            // WorldInspectorPlugin::new(),
            ))
        .run();
}
