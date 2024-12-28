use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct DamageEvent {
    pub source: Entity,
    pub target: Entity,
}
