use bevy::prelude::*;
use bevy_firefly::prelude::*;

use serenvale::{camera, game, game_state};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(FireflyPlugin)
        .init_state::<game_state::GameStates>()
        .add_plugins(camera::plugin::MainCameraPlugin)
        .add_plugins(game::plugin::GamePlugin)
        .run();
}
