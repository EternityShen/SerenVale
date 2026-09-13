use bevy::prelude::*;

use super::*;

/// 整个游戏界面和逻辑的插件集合
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(world::plugin::WorldPlugin);
    }
}
