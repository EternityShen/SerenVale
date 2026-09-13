use bevy::prelude::*;

/// 游戏状态
#[derive(States, Default, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum GameStates {
    #[default]
    GameIng,
}
