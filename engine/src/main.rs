use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use plugins::{core::CorePlugin, enemy::EnemyPlugin, map::MapPlugin, player::PlayerPlugin};

mod plugins;
mod state;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(CorePlugin)
        .add_plugins(MapPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
