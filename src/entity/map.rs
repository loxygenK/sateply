use std::collections::HashMap;

use ggez::GameResult;
use rand::{thread_rng, RngCore};

use super::{TypedEntity, Entity};

pub type EntityMapKey = u32;

#[derive(Default, Debug)]
pub struct EntityMap(HashMap<EntityMapKey, TypedEntity>);

pub struct EntityMapEntry<'a> {
    pub key: u32,
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

    pub fn update_all_entity(&mut self) -> GameResult {
        self.iter_mut_entity()
            .try_for_each(|entity| { entity.inner_mut().update() })
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
