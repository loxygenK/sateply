use std::collections::HashMap;

use crate::entity::RigidBody;
use crate::theory::physics::{PhysicalWorld, Physics, PhysicsController};
use ggez::graphics::ScreenImage;
use ggez::{graphics, Context, GameResult};
use rand::{thread_rng, RngCore};
use rapier2d::crossbeam::channel::internal::SelectHandle;

use super::{Entity, TypedEntity};

pub type EntityMapKey = u32;

#[derive(Debug)]
pub struct EntityMapValue {
    pub entity: TypedEntity,
    pub screen_image: ScreenImage,
}

#[derive(Default, Debug)]
pub struct EntityMap(HashMap<EntityMapKey, EntityMapValue>);

pub struct EntityMapEntry<'a> {
    pub key: EntityMapKey,
    pub value: &'a EntityMapValue,
}

impl EntityMap {
    pub fn inner(&self) -> &HashMap<EntityMapKey, EntityMapValue> {
        &self.0
    }

    pub fn iter_entity(&self) -> impl Iterator<Item = &EntityMapValue> {
        self.0.values()
    }

    pub fn iter_mut_entity(&mut self) -> impl Iterator<Item = &mut EntityMapValue> {
        self.0.values_mut()
    }

    pub fn update_all_entity(&mut self, ctx: &mut Context, physical_world: &mut PhysicalWorld) -> GameResult {
        self.iter_mut_entity()
            .try_for_each(|EntityMapValue { entity, .. }| {
                let Some(physics) = entity.as_mut_rigidbody() else {
                    return entity.inner_mut().update(ctx);
                };

                let mut controller = physical_world.get(physics.get_mut_physics()).unwrap();

                // TODO: This is not good I guess..
                controller.0.reset_forces(true);
                controller.0.reset_torques(true);
                physics.update_physics(&mut controller);

                Ok(())
            })?;

        physical_world.tick();

        self.iter_mut_entity()
            .for_each(|EntityMapValue { entity, .. }| {
                let Some(physics) = entity.as_mut_rigidbody() else {
                    return;
                };

                let controller = physical_world.get(physics.get_mut_physics()).unwrap();
                let transform = controller.to_transform();

                physics.report_transform(transform);

            });

        Ok(())
    }

    pub fn insert(
        &mut self,
        ctx: &Context,
        entity: TypedEntity,
    ) -> (&EntityMapKey, &EntityMapValue) {
        let mut rng = thread_rng();

        let mut key = rng.next_u32();
        while self.0.contains_key(&key) {
            key = rng.next_u32();
        }
        self.0.insert(
            key,
            EntityMapValue {
                entity,
                screen_image: graphics::ScreenImage::new(&ctx.gfx, None, 1.0, 1.0, 1),
            },
        );

        self.0.get_key_value(&key).unwrap()
    }
}

#[macro_export]
macro_rules! extract_by_entity {
    ($map: expr, $type: ident) => {{
        $map.iter_entity().flat_map(|entity| {
            if let $crate::entity::TypedEntity::$type(inner) = &entity.entity {
                Some(inner)
            } else {
                None
            }
        })
    }};

    (mut $map: expr, $type: ident) => {{
        $map.iter_mut_entity().flat_map(|entity| {
            if let $crate::entity::TypedEntity::$type(inner) = &mut entity.entity {
                Some(inner)
            } else {
                None
            }
        })
    }};
}
