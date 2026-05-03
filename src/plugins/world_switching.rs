use avian3d::prelude::Position;
use bevy::prelude::*;

use crate::{components::player::Player, plugins::{input::KeyBindings, level::{InsideWorld, OutsideWorld}}};

pub struct WorldSwitchingPlugin;

impl Plugin for WorldSwitchingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_key_and_go_inside);
        app.insert_resource(ActiveWorld::Outside);
        app.init_resource::<OutsideCheckpoint>();
        app.add_observer(on_moved_outside);
    }
}

#[derive(Resource, Default)]
struct OutsideCheckpoint {
    transform: Option<Transform>
}

#[derive(Resource)]
enum ActiveWorld {
    Outside,
    Inside,
}

fn check_key_and_go_inside(
    keys: Res<ButtonInput<KeyCode>>,
    keybinds: Res<KeyBindings>,
    mut active_world: ResMut<ActiveWorld>,
    player_transform: Single<&Transform, With<Player>>,
    mut outside_checkpoint: ResMut<OutsideCheckpoint>,
    mut outside_world_visiblity: Single<&mut Visibility, (With<OutsideWorld>, Without<InsideWorld>)>,
    mut inside_world_visiblity: Single<&mut Visibility, (With<InsideWorld>, Without<OutsideWorld>)>,
) {
    if keys.just_released((*keybinds).retract_to_shell) {
        if let ActiveWorld::Outside = *active_world {
            println!("we're inside now");

            **outside_world_visiblity = Visibility::Hidden;
            **inside_world_visiblity = Visibility::Visible;

            // first dereference through Single, second deref to get Transform from &Transform
            outside_checkpoint.transform = Some(**player_transform);

            *active_world = ActiveWorld::Inside;
        }
    }
}

#[derive(Event)]
pub struct MovedOutside;

fn on_moved_outside(
    _event: On<MovedOutside>,
    mut active_world: ResMut<ActiveWorld>,
    mut player: Single<(&mut Transform, &mut Position), With<Player>>,
    mut outside_checkpoint: ResMut<OutsideCheckpoint>,
    mut outside_world_visibility: Single<&mut Visibility, (With<OutsideWorld>, Without<InsideWorld>)>,
    mut inside_world_visibility: Single<&mut Visibility, (With<InsideWorld>, Without<OutsideWorld>)>,
) {
    if let ActiveWorld::Inside = *active_world {
        println!("we're outside now");

        **outside_world_visibility = Visibility::Visible;
        **inside_world_visibility =  Visibility::Hidden;

        let target = outside_checkpoint.transform.take().unwrap().translation;
        player.0.translation = target;
        *player.1 = Position(target);

        *active_world = ActiveWorld::Outside;
    }
}