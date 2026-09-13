use bevy::prelude::*;
use bevy_ecs_ldtk::LevelSelection;

use super::systems;

use crate::game_state;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::index(0));
        app.add_systems(OnEnter(game_state::GameStates::GameIng), systems::setup);
    }
}
