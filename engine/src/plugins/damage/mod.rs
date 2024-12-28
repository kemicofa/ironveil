use bevy::{
    app::{App, Plugin, Update},
    prelude::*,
};

use crate::{collision_state::CollisionState, events::DamageEvent, health::Health, state::AppState};

use super::{enemy::Enemy, player::Player};

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_damage).run_if(in_state(AppState::InGame)));
    }
}

fn handle_damage(
    time: Res<Time>,
    mut player_query: Query<&mut Health, With<Player>>,
    enemy_query: Query<&Enemy>,
    mut active_collisions: ResMut<CollisionState>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        for (enemy_entity, elapsed_time) in active_collisions.colliding_entities.iter_mut() {
            if let Ok(enemy) = enemy_query.get(*enemy_entity) {
                let delta_time = time.delta_secs();
                *elapsed_time += delta_time;
                let damage = enemy.damage_per_second * delta_time;
                player.apply_damage(damage);
                println!(
                    "Player takes {:.2} damage from enemy {:?}. Health is now {:.2}.",
                    damage, enemy_entity, player.current
                );
            }
        }
    }
}
