use bevy::{
    app::{App, Plugin, Update},
    asset::AssetServer,
    color::Color,
    input::ButtonInput,
    math::{Vec2, Vec3},
    prelude::*,
    sprite::Sprite,
    time::Time,
    window::{PrimaryWindow, Window, WindowResized},
};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, GravityScale, LockedAxes, RigidBody};

use crate::{health::Health, state::{AppState, GameState}};

use super::map::Map;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            (setup, resize_player_on_window_resize),
        )
        .add_systems(
            Update,
            (
                check_health,
                movement,
                camera_follow,
                resize_player_on_window_resize,
                update_health_bar,
            )
                .run_if(in_state(GameState::Ongoing)),
        );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MovementSpeed(f32);

#[derive(Component)]
pub struct HealthBar;

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
        .spawn(Sprite::from_image(asset_server.load("player.png")))
        .insert(Player)
        .insert(Transform {
            translation: Vec3::new(center_x, center_y, 1.0), // Position at the center
            scale: Vec3::splat(scale_factor),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(16.0, 16.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(GravityScale(0.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Health::new(100.0))
        .insert(MovementSpeed(200.0))
        .with_children(|parent| {
            parent
                .spawn(Sprite {
                    color: Color::linear_rgb(1.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(32.0, 4.0)),
                    ..default()
                })
                .insert(Transform::from_xyz(0.0, 24.0, 0.0))
                .insert(HealthBar);
        });
}

fn check_health(
    mut next_game_state: ResMut<NextState<GameState>>,    
    player_query: Query<&Health, With<Player>>,
) {
    if let Ok(player) = player_query.get_single() {
        if player.is_dead() {
            next_game_state.set(GameState::GameOver);
        }
    }
}

fn update_health_bar(
    players: Query<(&Health, &Children), With<Player>>,
    mut health_bars: Query<&mut Sprite, With<HealthBar>>,
) {
    for (health, children) in players.iter() {
        for &child in children.iter() {
            if let Ok(mut sprite) = health_bars.get_mut(child) {
                // Adjust the width of the health bar based on health percentage
                let health_percentage = health.current.max(0.0) / health.max;
                sprite.custom_size = Some(Vec2::new(32.0 * health_percentage as f32, 4.0));
            }
        }
    }
}

fn movement(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &MovementSpeed), With<Player>>
) {
    for (mut transform, movement_speed) in &mut query {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        if input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }

        if direction != Vec3::ZERO {
            transform.translation += direction.normalize() * movement_speed.0 * time.delta_secs();
        }
    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            // Match the camera's position with the player's position
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}

fn resize_player_on_window_resize(
    mut resize_events: EventReader<WindowResized>,
    mut query: Query<&mut Transform, With<Player>>,
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
