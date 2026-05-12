use bevy::prelude::*;
#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum ActivityState {
    #[default]
    Playing,
    Cutscene,
    Pause,
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum LocationState {
    #[default]
    Outside,
    Inside,
}
