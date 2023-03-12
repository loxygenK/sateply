use std::collections::HashMap;

use ggez::{GameResult, graphics::Canvas, glam::Vec2};
use rand::{thread_rng, RngCore};

use crate::GameState;

pub mod satelite;

pub trait Entity {
    fn update(&mut self) -> GameResult;
    fn draw(&self, canvas: &mut Canvas, state: &GameState) -> GameResult<Vec2>;
    fn get_type(&self) -> EntityType;
}

pub enum EntityType {
    Satelite
}

pub type EntityMapKey = u32;

pub struct EntityMap(HashMap<EntityMapKey, Box<dyn Entity>>);

pub struct EntityMapEntry<'a> {
    pub key: EntityMapKey,
    pub entity: &'a dyn Entity
}

impl EntityMap {
    pub fn inner(&self) -> &HashMap<EntityMapKey, Box<dyn Entity>> {
        &self.0
    }

    pub fn iter_entity(&self) -> impl Iterator<Item = &Box<dyn Entity>> {
        self.0.values()
    }

    pub fn insert(&mut self, entity: Box<dyn Entity>) -> EntityMapEntry {
        let mut rng = thread_rng();

        let mut key = rng.next_u32();
        while self.0.contains_key(&key) {
            key = rng.next_u32();
        }
        self.0.insert(key, entity);

        EntityMapEntry {
            key,
            entity: self.0.get(&key).unwrap().as_ref()
        }
    }
}
