use avian3d::prelude::Position;
use bevy::prelude::*;

use crate::{components::{gamestate::LocationState, player::Player}, plugins::{input::KeyBindings, level::{InsideWorld, OutsideWorld}}};

pub struct WorldSwitchingPlugin;

impl Plugin for WorldSwitchingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,check_for_retract);
        app.init_resource::<OutsideCheckpoint>();
        app.add_observer(on_moved_outside);
        app.add_observer(on_moved_inside);
        app.add_systems(OnEnter(LocationState::Outside), (show_outside, hide_inside));
        app.add_systems(OnEnter(LocationState::Inside), (show_inside, hide_outside));
        app.add_systems(PostStartup, (show_outside, hide_inside));
    }
}

#[derive(Resource, Default)]
struct OutsideCheckpoint {
    transform: Option<Transform>
}

fn check_for_retract(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    keybinds: Res<KeyBindings>,
) {
    if keys.just_released((*keybinds).retract_to_shell) {
        commands.trigger(MovedInside);
    }
}

#[derive(Event)]
struct MovedInside;

fn on_moved_inside(
    _event: On<MovedInside>,
    current_location: Res<State<LocationState>>,
    mut next_location: ResMut<NextState<LocationState>>,
    mut player: Single<(&mut Transform, &mut Position), With<Player>>,
    mut outside_checkpoint: ResMut<OutsideCheckpoint>,
) {
    if *current_location == LocationState::Outside {
        println!("we're inside now");
        // first dereference through Single, second deref to get Transform from &Transform
        outside_checkpoint.transform = Some(*(player.0));
        let target = Vec3::from([0.0, 0.0, 5.0]);
        player.0.translation = target;
        *player.1 = Position(target);

        next_location.set(LocationState::Inside);
    }
}

#[derive(Event)]
pub struct MovedOutside;

fn on_moved_outside(
    _event: On<MovedOutside>,
    current_location: Res<State<LocationState>>,
    mut next_location: ResMut<NextState<LocationState>>,
    mut player: Single<(&mut Transform, &mut Position), With<Player>>,
    mut outside_checkpoint: ResMut<OutsideCheckpoint>,
) {
    if *current_location == LocationState::Inside {
        println!("we're outside now");
        let target = outside_checkpoint.transform.take().unwrap().translation;
        player.0.translation = target;
        *player.1 = Position(target);

        next_location.set(LocationState::Outside);
    }
}

fn show_outside(mut commands: Commands, query: Query<Entity, With<OutsideWorld>>) {
    println!("hello");
    for entity in query {
        commands.entity(entity).insert(Visibility::Visible);
    }
}
fn hide_outside(mut commands: Commands, query: Query<Entity, With<OutsideWorld>>) {
    for entity in query {
        commands.entity(entity).insert(Visibility::Hidden);
    }
}
fn show_inside(mut commands: Commands, query: Query<Entity, With<InsideWorld>>) {
    for entity in query {
        commands.entity(entity).insert(Visibility::Visible);
    }
}
fn hide_inside(mut commands: Commands, query: Query<Entity, With<InsideWorld>>) {
    println!("bye");
    for entity in query {
        commands.entity(entity).insert(Visibility::Hidden);
    }
}