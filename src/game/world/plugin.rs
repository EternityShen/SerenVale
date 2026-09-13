use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use super::map;

/// 世界插件
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin);
        app.add_plugins(map::plugin::MapPlugin);
    }
}
