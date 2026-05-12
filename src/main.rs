use avian3d::prelude::*;
use bevy::{audio::AudioPlugin, prelude::*, window::WindowResolution};
mod components;
mod plugins;

use plugins::{
    camera::CameraPlugin, input::InputPlugin, level::WorldPlugin, player::PlayerPlugin,
    shooting::ShootingPlugin, world_switching::WorldSwitchingPlugin,
};

use crate::{
    components::gamestate::{ActivityState, LocationState},
    plugins::{cutscene::CutscenePlugin, inventory::InventoryPlugin},
};
fn main() {
    App::new()
        .add_plugins((DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(1920, 1080).with_scale_factor_override(1.0),
                    position: WindowPosition::Centered(MonitorSelection::Index(1)),
                    ..default()
                }),
                ..default()
            })
            .disable::<AudioPlugin>(),))
        .init_state::<ActivityState>()
        .init_state::<LocationState>()
        .add_plugins((
            PhysicsPlugins::default(),
            //PhysicsDebugPlugin::default(),
            CutscenePlugin,
            InputPlugin,
            PlayerPlugin,
            CameraPlugin,
            WorldPlugin,
            ShootingPlugin,
            WorldSwitchingPlugin,
            InventoryPlugin,
            // Inspector
            // EguiPlugin::default(),
            // WorldInspectorPlugin::new(),
        ))
        .run();
}
