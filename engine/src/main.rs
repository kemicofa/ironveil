use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use events::DamageEvent;
use plugins::{
    collision::CollisionPlugin, damage::DamagePlugin, enemy::EnemyPlugin, game_over::GameOverPlugin, map::MapPlugin, menu::MenuPlugin, player::PlayerPlugin
};
use state::{AppState, GameState};

mod events;
mod plugins;
mod state;
mod health;
mod collision_state;

fn main() {
    App::new()
        .add_event::<DamageEvent>() 
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(MenuPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(DamagePlugin)
        .add_plugins(GameOverPlugin)
        .add_systems(Startup, setup)
        .init_state::<AppState>()
        .init_state::<GameState>()
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
