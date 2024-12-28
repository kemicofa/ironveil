use bevy::{
    app::{App, Plugin, Update},
    asset::AssetServer,
    math::{Vec2, Vec3},
    prelude::{
        in_state, Commands, Component, Entity, EventReader, IntoSystemConfigs, OnEnter, Query, Res,
        Resource, Transform, With,
    },
    sprite::Sprite,
    time::Time,
    window::{PrimaryWindow, Window, WindowResized},
};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, GravityScale, LockedAxes, RigidBody};

use crate::state::AppState;

use super::{map::Map, player::Player};

#[derive(Component)]
pub struct Enemy {
   pub damage_per_second: f32
}

#[derive(Component)]
struct Velocity(Vec2);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            (setup, resize_enemy_on_window_resize),
        )
        .add_systems(
            Update,
            (
                resize_enemy_on_window_resize,
                move_toward_player,
                update_position,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}

fn setup(
    mut commands: Commands,
    map: Res<Map>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Calculate the center of the map
    let center_x = map.width as f32 / 2.0;
    let center_y = map.height as f32 / 2.0;

    let window = window_query.single();

    let scale_factor = calculate_scale(window);

    // Player entity
    commands
        .spawn(Sprite::from_image(asset_server.load("enemy.png")))
        .insert(Enemy {
            damage_per_second: 10.0
        })
        .insert(Transform {
            translation: Vec3::new(center_x, center_y, 1.0), // Position at the center
            scale: Vec3::splat(scale_factor),
            ..Default::default()
        })
        .insert(Velocity(Vec2::ZERO))
        .insert(MovementSpeed(90.0))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(16.0, 16.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(GravityScale(0.0))
        .insert(LockedAxes::ROTATION_LOCKED);
}

#[derive(Component)]
pub struct MovementSpeed(f32);

fn move_toward_player(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&Transform, &mut Velocity, &MovementSpeed), With<Enemy>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (enemy_transform, mut velocity, movement_speed) in enemy_query.iter_mut() {
            // Calculate direction vector towards the player
            let direction =
                player_transform.translation.truncate() - enemy_transform.translation.truncate();

            // Normalize direction to get a unit vector
            let normalized_direction = direction.normalize_or_zero();

            // Update velocity (adjust speed as needed)
            velocity.0 = normalized_direction * movement_speed.0;
        }
    }
}

fn update_position(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity), With<Enemy>>) {
    for (mut transform, velocity) in query.iter_mut() {
        // Update position based on velocity and delta time
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    }
}

fn resize_enemy_on_window_resize(
    mut resize_events: EventReader<WindowResized>,
    mut query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Process all window resize events
    for _ in resize_events.read() {
        if let Ok(mut player_transform) = query.get_single_mut() {
            if let Ok(window) = window_query.get_single() {
                // Calculate new scale factor based on window dimensions
                let scale_factor = calculate_scale(window);
                player_transform.scale = Vec3::splat(scale_factor);
            }
        }
    }
}

fn calculate_scale(window: &Window) -> f32 {
    // Example: Base scale on the smaller of the two dimensions
    let smallest_dimension = window.width().min(window.height());
    let base_size = 500.0; // Adjust this value to match the desired base size of your player
    smallest_dimension / base_size
}
