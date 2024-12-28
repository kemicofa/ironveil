use bevy::{prelude::{Entity, Resource}, utils::{HashMap, HashSet}};

#[derive(Resource)]
pub struct CollisionState {
   pub colliding_entities: HashMap<Entity, f32>,
}

impl Default for CollisionState {
    fn default() -> Self {
        Self { colliding_entities: Default::default() }
    }
}
