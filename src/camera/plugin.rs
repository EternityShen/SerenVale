use bevy::prelude::*;

use crate::camera::systems;

/// 世界主相机插件
pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::setup);
        app.add_systems(Update, systems::follow_player);
    }
}
