use bevy::prelude::*;
use avian3d::prelude::*;
use std::collections::VecDeque;


use crate::components::gamestate::{ActivityState, LocationState};
use crate::components::player::Player;

pub struct CutscenePlugin;

impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CutsceneSequence>();
        app.add_systems(Update, process_cutscene.run_if(in_state(ActivityState::Cutscene)));
    }
}

#[derive(Clone)]
pub enum CutsceneAction {
    Wait(f32),
    MoveCamera(Vec3, f32),
    MoveCameraToPlayer,
    ZoomCamera(f32),
    MovePlayer(Vec3),
    NextLevel(LocationState)
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
) {
    loop {
        let action = match cutscene_sequence.actions.front() {
            Some(a) => a.clone(),
            None => {
                next_activity.set(ActivityState::Playing);
                return;
            }
        };
        
        let mut is_finished = true;
        match action {
            CutsceneAction::Wait(duration) => {
                let timer = cutscene_sequence.timer.get_or_insert(Timer::from_seconds(duration, TimerMode::Once));
                timer.tick(time.delta());
                is_finished = timer.is_finished();
            },
            CutsceneAction::MovePlayer(target) => {
                player.0.translation = target;
                *player.1 = Position(target);
            },
            CutsceneAction::NextLevel(next_level) => {
                next_location.set(next_level);
            },
            CutsceneAction::MoveCameraToPlayer => {
                // Todo: move camera magic numbers scattered across systems to a single CameraSettings resource (height, lerp_speed, zoom, etc)
                camera_transform.translation = Vec3::new(player.0.translation.x, 10.0, player.0.translation.z + 10.0);
            }
            _ => {
                println!("action not implemented yet");

            }
        }
        if is_finished {
            cutscene_sequence.actions.pop_front();
            cutscene_sequence.timer = None;
        } else {
            return;
        }
    }
}
