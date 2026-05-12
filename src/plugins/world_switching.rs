use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    components::{
        gamestate::{ActivityState, LocationState},
        player::Player,
    },
    plugins::{
        cutscene::CutsceneAction,
        cutscene::CutsceneSequence,
        input::KeyBindings,
        level::{InsideWorld, OutsideWorld},
    },
};

pub struct WorldSwitchingPlugin;

impl Plugin for WorldSwitchingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_for_retract);
        app.init_resource::<OutsideCheckpoint>();
        app.add_observer(on_moved_outside);
        app.add_observer(on_moved_inside);
        app.add_systems(
            OnEnter(LocationState::Inside),
            (
                show_inside,
                hide_outside,
                enable_physics_inside,
                disable_physics_outside,
            ),
        );
        app.add_systems(
            OnEnter(LocationState::Outside),
            (
                show_outside,
                hide_inside,
                enable_physics_outside,
                disable_physics_inside,
            ),
        );
        app.add_systems(
            PostStartup,
            (
                show_outside,
                hide_inside,
                enable_physics_outside,
                disable_physics_inside,
            ),
        );
    }
}

#[derive(Resource, Default)]
struct OutsideCheckpoint {
    transform: Option<Transform>,
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
    current_activity: Res<State<ActivityState>>,
    mut next_activity: ResMut<NextState<ActivityState>>,
    mut outside_checkpoint: ResMut<OutsideCheckpoint>,
    mut cutscene: ResMut<CutsceneSequence>,
    player: Single<&mut Transform, With<Player>>,
) {
    if *current_location == LocationState::Outside && *current_activity == ActivityState::Playing {
        outside_checkpoint.transform = Some(**player);
        next_activity.set(ActivityState::Cutscene);
        cutscene.actions.push_back(CutsceneAction::FadeAndZoom {
            amount: 8.,
            duration: 0.4,
            start: None,
            reversed: false,
        });
        cutscene.actions.push_back(CutsceneAction::Wait(0.2));
        cutscene
            .actions
            .push_back(CutsceneAction::NextLevel(LocationState::Inside));
        cutscene
            .actions
            .push_back(CutsceneAction::MovePlayer(Vec3::from([0.0, 0.0, 5.0])));
        cutscene
            .actions
            .push_back(CutsceneAction::MoveCameraToPlayer);
        cutscene.actions.push_back(CutsceneAction::FadeAndZoom {
            amount: 8.,
            duration: 0.4,
            start: None,
            reversed: true,
        });
    }
}

#[derive(Event)]
pub struct MovedOutside;

fn on_moved_outside(
    _event: On<MovedOutside>,
    current_location: Res<State<LocationState>>,
    mut outside_checkpoint: ResMut<OutsideCheckpoint>,
    current_activity: Res<State<ActivityState>>,
    mut next_activity: ResMut<NextState<ActivityState>>,
    mut cutscene: ResMut<CutsceneSequence>,
) {
    if *current_location == LocationState::Inside && *current_activity == ActivityState::Playing {
        let target = outside_checkpoint.transform.take().unwrap().translation;
        next_activity.set(ActivityState::Cutscene);
        cutscene.actions.push_back(CutsceneAction::FadeAndZoom {
            amount: 8.,
            duration: 0.4,
            start: None,
            reversed: false,
        });
        cutscene
            .actions
            .push_back(CutsceneAction::NextLevel(LocationState::Outside));
        cutscene
            .actions
            .push_back(CutsceneAction::MovePlayer(target));
        cutscene
            .actions
            .push_back(CutsceneAction::MoveCameraToPlayer);
        cutscene.actions.push_back(CutsceneAction::FadeAndZoom {
            amount: 8.,
            duration: 0.4,
            start: None,
            reversed: true,
        });
        cutscene.actions.push_back(CutsceneAction::Wait(0.2));
    }
}

fn show_outside(
    mut commands: Commands,
    query: Query<Entity, (With<OutsideWorld>, With<Visibility>)>,
) {
    for entity in query {
        commands.entity(entity).insert(Visibility::Visible);
    }
}
fn hide_outside(
    mut commands: Commands,
    query: Query<Entity, (With<OutsideWorld>, With<Visibility>)>,
) {
    for entity in query {
        commands.entity(entity).insert(Visibility::Hidden);
    }
}
fn show_inside(
    mut commands: Commands,
    query: Query<Entity, (With<InsideWorld>, With<Visibility>)>,
) {
    for entity in query {
        commands.entity(entity).insert(Visibility::Visible);
    }
}
fn hide_inside(
    mut commands: Commands,
    query: Query<Entity, (With<InsideWorld>, With<Visibility>)>,
) {
    for entity in query {
        commands.entity(entity).insert(Visibility::Hidden);
    }
}

fn enable_physics_outside(
    mut commands: Commands,
    query: Query<
        Entity,
        (
            With<OutsideWorld>,
            With<RigidBodyDisabled>,
            With<ColliderDisabled>,
        ),
    >,
) {
    for entity in query {
        commands
            .entity(entity)
            .remove::<(RigidBodyDisabled, ColliderDisabled)>();
    }
}
fn disable_physics_outside(
    mut commands: Commands,
    query: Query<Entity, (With<OutsideWorld>, With<RigidBody>, With<Collider>)>,
) {
    for entity in query {
        commands
            .entity(entity)
            .insert((RigidBodyDisabled, ColliderDisabled));
    }
}
fn enable_physics_inside(
    mut commands: Commands,
    query: Query<
        Entity,
        (
            With<InsideWorld>,
            With<RigidBodyDisabled>,
            With<ColliderDisabled>,
        ),
    >,
) {
    for entity in query {
        commands
            .entity(entity)
            .remove::<(RigidBodyDisabled, ColliderDisabled)>();
    }
}
fn disable_physics_inside(
    mut commands: Commands,
    query: Query<Entity, (With<InsideWorld>, With<RigidBody>, With<Collider>)>,
) {
    for entity in query {
        commands
            .entity(entity)
            .insert((RigidBodyDisabled, ColliderDisabled));
    }
}
