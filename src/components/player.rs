use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSet {
    Movement,
}
