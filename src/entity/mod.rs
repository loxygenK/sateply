use std::collections::HashMap;

use ggez::{GameResult, graphics::Canvas, glam::Vec2};
use rand::{thread_rng, RngCore};

use crate::GameState;

use self::satelite::Satelite;

pub mod satelite;

pub trait Entity {
    fn update(&mut self) -> GameResult;
    fn draw(&self, canvas: &mut Canvas, state: &GameState) -> GameResult<Vec2>;
    fn typed(self) -> TypedEntity;
}

pub enum TypedEntity {
    Satelite(Satelite)
}

impl TypedEntity {
    pub fn inner(&self) -> &impl Entity {
        match self {
            TypedEntity::Satelite(inner) => inner
        }
    }

    pub fn inner_mut(&mut self) -> &mut impl Entity {
        match self {
            TypedEntity::Satelite(inner) => inner
        }
    }
}

pub type EntityMapKey = u32;

#[derive(Default)]
pub struct EntityMap(HashMap<EntityMapKey, TypedEntity>);

pub struct EntityMapEntry<'a> {
    pub key: EntityMapKey,
    pub entity: &'a TypedEntity
}

impl EntityMap {
    pub fn inner(&self) -> &HashMap<EntityMapKey, TypedEntity> {
        &self.0
    }

    pub fn iter_entity(&self) -> impl Iterator<Item = &TypedEntity> {
        self.0.values()
    }

    pub fn iter_mut_entity(&mut self) -> impl Iterator<Item = &mut TypedEntity> {
        self.0.values_mut()
    }

    pub fn insert(&mut self, entity: TypedEntity) -> EntityMapEntry {
        let mut rng = thread_rng();

        let mut key = rng.next_u32();
        while self.0.contains_key(&key) {
            key = rng.next_u32();
        }
        self.0.insert(key, entity);

        EntityMapEntry {
            key,
            entity: self.0.get(&key).unwrap()
        }
    }
}

#[macro_export]
macro_rules! extract_by_entity {
    ($map: expr, $type: ident) => {
        {
            $map.iter_entity()
                .flat_map(|entity| {
                    if let $crate::entity::TypedEntity::$type(inner) = entity {
                        Some(inner)
                    } else {
                        None
                    }
                })
        }
    };

    (mut $map: expr, $type: ident) => {
        {
            $map.iter_mut_entity()
                .flat_map(|entity| {
                    if let $crate::entity::TypedEntity::$type(inner) = entity {
                        Some(inner)
                    } else {
                        None
                    }
                })
        }
    };
}
