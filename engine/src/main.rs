use bevy::prelude::*;
use plugins::{core::CorePlugin, map::MapPlugin, player::PlayerPlugin};

mod plugins;
mod state;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins(CorePlugin)
        .add_plugins(MapPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
