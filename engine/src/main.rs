use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use events::DamageEvent;
use plugins::{
    collision::CollisionPlugin, core::CorePlugin, damage::DamagePlugin, enemy::EnemyPlugin, map::MapPlugin, player::PlayerPlugin
};

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
        .add_plugins(CorePlugin)
        .add_plugins(MapPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(DamagePlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
