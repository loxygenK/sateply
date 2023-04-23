use std::collections::HashMap;

use crate::entity::TypedEntity;
use crate::theory::physics::PhysicalWorld;
use ggez::graphics::ScreenImage;
use ggez::{graphics, Context, GameResult};
use rand::{thread_rng, RngCore};

pub type WorldKey = u32;

#[derive(Debug)]
pub struct WorldValue {
    pub entity: TypedEntity,
    pub screen_image: ScreenImage,
}

#[derive(Default, Debug)]
pub struct World {
    map: HashMap<WorldKey, WorldValue>,
    physical_world: PhysicalWorld,
}

pub struct EntityMapEntry<'a> {
    pub key: WorldKey,
    pub value: &'a WorldValue,
}

impl World {
    pub fn iter_mut_entity(&mut self) -> impl Iterator<Item = &mut WorldValue> {
        self.map.values_mut()
    }

    pub fn update_all_entity(&mut self, ctx: &mut Context) -> GameResult {
        self.map
            .values_mut()
            .try_for_each(|WorldValue { entity, .. }| {
                let Some(physics) = entity.as_mut_rigidbody() else {
                    return entity.inner_mut().update(ctx);
                };

                let mut controller = self.physical_world.get(physics.get_mut_physics()).unwrap();

                // TODO: This is not good I guess..
                controller.0.reset_forces(true);
                controller.0.reset_torques(true);
                physics.update_physics(&mut controller);

                Ok(())
            })?;

        self.physical_world.tick();

        self.map.values_mut().for_each(|WorldValue { entity, .. }| {
            let Some(physics) = entity.as_mut_rigidbody() else {
                    return;
                };

            let controller = self.physical_world.get(physics.get_mut_physics()).unwrap();
            let transform = controller.to_transform();

            physics.report_transform(transform);
        });

        Ok(())
    }

    pub fn get_mut(&mut self, key: &WorldKey) -> Option<&mut WorldValue> {
        self.map.get_mut(key)
    }

    pub fn insert(&mut self, ctx: &Context, mut entity: TypedEntity) -> (&WorldKey, &WorldValue) {
        let mut rng = thread_rng();

        if let Some(physics_impl) = entity.as_mut_rigidbody() {
            let physics_handle = self.physical_world.register(physics_impl.get_property());
            physics_impl.register_physics(physics_handle);
        }

        let mut key = rng.next_u32();
        while self.map.contains_key(&key) {
            key = rng.next_u32();
        }
        self.map.insert(
            key,
            WorldValue {
                entity,
                screen_image: graphics::ScreenImage::new(&ctx.gfx, None, 1.0, 1.0, 1),
            },
        );

        self.map.get_key_value(&key).unwrap()
    }
}

#[macro_export]
macro_rules! as_type {
    (& $entity: expr, $type: ident) => {
        #[allow(irrefutable_let_patterns)]
        if let $crate::entity::TypedEntity::$type(inner) = &$entity {
            Some(inner)
        } else {
            None
        }
    };

    (&mut $entity: expr, $type: ident) => {
        #[allow(irrefutable_let_patterns)]
        if let $crate::entity::TypedEntity::$type(inner) = &mut $entity {
            Some(inner)
        } else {
            None
        }
    };
}
