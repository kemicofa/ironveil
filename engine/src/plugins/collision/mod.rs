use bevy::{prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{collision_state::CollisionState, events::DamageEvent, state::AppState};

use super::{enemy::Enemy, player::Player};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(CollisionState::default())
        .add_systems(Update, handle_collisions.run_if(in_state(AppState::InGame)));
    }
}


fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    mut collision_state: ResMut<CollisionState>,
) { 
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(entity1, entity2, _) => {
                let player_entity = if player_query.get(*entity1).is_ok() {
                    Some(*entity1)
                } else if player_query.get(*entity2).is_ok() {
                    Some(*entity2)
                } else {
                    None
                };

                let enemy_entity = if enemy_query.get(*entity1).is_ok() {
                    Some(*entity1)
                } else if enemy_query.get(*entity2).is_ok() {
                    Some(*entity2)
                } else {
                    None
                };

                if let (Some(player), Some(enemy)) = (player_entity, enemy_entity) {
                    if player != enemy {
                        collision_state.colliding_entities.insert(enemy, 0.0);
                    }
                }
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {
                let player_entity = if player_query.get(*entity1).is_ok() {
                    Some(*entity1)
                } else if player_query.get(*entity2).is_ok() {
                    Some(*entity2)
                } else {
                    None
                };

                let enemy_entity = if enemy_query.get(*entity1).is_ok() {
                    Some(*entity1)
                } else if enemy_query.get(*entity2).is_ok() {
                    Some(*entity2)
                } else {
                    None
                };

                if let (Some(player), Some(enemy)) = (player_entity, enemy_entity) {
                    if player != enemy {
                        collision_state.colliding_entities.remove(&enemy);
                    }
                }
            }
        }
    }
}
