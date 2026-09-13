use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use super::components::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle = asset_server.load("map.ldtk");

    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle: map_handle.into(),
            transform: Transform::from_xyz(16.0, 16.0, 1.0),
            ..Default::default()
        },
        WorldMap,
    ));

    info!("创建世界地图");
}
