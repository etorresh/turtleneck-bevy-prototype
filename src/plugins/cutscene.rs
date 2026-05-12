use avian3d::prelude::*;
use bevy::prelude::*;
use std::collections::VecDeque;

use crate::components::gamestate::{ActivityState, LocationState};
use crate::components::player::Player;

pub struct CutscenePlugin;

impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_fade_overlay);
        app.init_resource::<CutsceneSequence>();
        app.add_systems(
            Update,
            process_cutscene.run_if(in_state(ActivityState::Cutscene)),
        );
    }
}

#[derive(Component)]
pub struct FadeOverlay;

fn spawn_fade_overlay(mut commands: Commands) {
    commands.spawn((
        FadeOverlay,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        ZIndex(999),
    ));
}

#[derive(Clone)]
pub enum CutsceneAction {
    Wait(f32),
    MoveCamera(Vec3, f32),
    MoveCameraToPlayer,
    ZoomCamera(f32, f32, Option<Vec3>),
    FadeAndZoom {
        amount: f32,
        duration: f32,
        start: Option<Vec3>,
        reversed: bool,
    },
    MovePlayer(Vec3),
    NextLevel(LocationState),
    FadeToBlack,
}

#[derive(Resource, Default)]
pub struct CutsceneSequence {
    pub actions: VecDeque<CutsceneAction>,
    timer: Option<Timer>,
}

fn process_cutscene(
    time: Res<Time>,
    mut cutscene_sequence: ResMut<CutsceneSequence>,
    mut next_location: ResMut<NextState<LocationState>>,
    mut next_activity: ResMut<NextState<ActivityState>>,
    mut player: Single<(&mut Transform, &mut Position), With<Player>>,
    mut camera_transform: Single<&mut Transform, (With<Camera3d>, Without<Player>)>,
    mut overlay: Single<&mut BackgroundColor, With<FadeOverlay>>,
) {
    // https://bevy-cheatbook.github.io/pitfalls/split-borrows.html
    let cutscene_sequence = &mut *cutscene_sequence;
    loop {
        let action = match cutscene_sequence.actions.front_mut() {
            Some(a) => a,
            None => {
                next_activity.set(ActivityState::Playing);
                return;
            }
        };

        match action {
            CutsceneAction::Wait(duration) => {
                let timer = cutscene_sequence
                    .timer
                    .get_or_insert(Timer::from_seconds(*duration, TimerMode::Once));
                timer.tick(time.delta());
            }
            CutsceneAction::MovePlayer(target) => {
                player.0.translation = *target;
                *player.1 = Position(*target);
            }
            CutsceneAction::NextLevel(next_level) => {
                next_location.set(next_level.clone());
            }
            CutsceneAction::MoveCameraToPlayer => {
                // Todo: move camera magic numbers scattered across systems to a single CameraSettings resource (height, lerp_speed, zoom, etc)
                camera_transform.translation =
                    Vec3::new(player.0.translation.x, 10.0, player.0.translation.z + 10.0);
            }
            CutsceneAction::ZoomCamera(amount, duration, start) => {
                let start = start.get_or_insert(camera_transform.translation);
                let timer = cutscene_sequence
                    .timer
                    .get_or_insert(Timer::from_seconds(*duration, TimerMode::Once));
                timer.tick(time.delta());
                let t = (timer.elapsed_secs() / *duration).clamp(0.0, 1.0);
                let smooth_t = t * t * (3.0 - 2.0 * t);
                let target = Vec3::new(
                    player.0.translation.x,
                    10. - *amount,
                    player.0.translation.z + 10. - *amount,
                );
                camera_transform.translation = start.lerp(target, smooth_t);
            }
            CutsceneAction::FadeAndZoom {
                amount,
                duration,
                start,
                reversed,
            } => {
                let start = start.get_or_insert(camera_transform.translation);
                let timer = cutscene_sequence
                    .timer
                    .get_or_insert(Timer::from_seconds(*duration, TimerMode::Once));
                timer.tick(time.delta());
                let t = (timer.elapsed_secs() / *duration).clamp(0.0, 1.0);
                let smooth_t = t * t * (3.0 - 2.0 * t);
                let smooth_t = if *reversed { 1.0 - smooth_t } else { smooth_t };
                let target = Vec3::new(
                    player.0.translation.x,
                    10. - *amount,
                    player.0.translation.z + 10. - *amount,
                );
                camera_transform.translation = start.lerp(target, smooth_t);
                overlay.0 = Color::srgba(0.0, 0.0, 0.0, smooth_t);
            }
            _ => {
                println!("action not implemented yet");
            }
        }

        if cutscene_sequence
            .timer
            .as_ref()
            .is_some_and(|timer| !timer.is_finished())
        {
            return;
        } else {
            cutscene_sequence.actions.pop_front();
            cutscene_sequence.timer = None;
        }
    }
}
