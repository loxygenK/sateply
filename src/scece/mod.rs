use ggez::{input::keyboard::KeyInput, Context};

use crate::entity::map::EntityMap;
use crate::entity::RigidBody;
use crate::{
    entity::{satelite::Satelite, Entity},
    extract_by_entity,
    system::{keyinput_list::KeyTypeMatchMap, state::GameState},
    theory::physics::Physics,
    utils::ExpectOnlyOneExt,
};

pub mod game;

pub trait Scene {
    fn prepare(&self, ctx: &Context, state: &mut GameState, entity_map: &mut EntityMap);
    fn tick(
        &self,
        ctx: &Context,
        state: &mut GameState,
        entity_map: &mut EntityMap,
    ) -> Option<SceneTickAction>;
}

pub enum SceneTickAction {
    ChangeScene(Scenes),
}

pub enum Scenes {
    DefaultScene(DefaultScene),
    GameScene(game::GameScene),
}

impl Scenes {
    pub fn inner(&self) -> &dyn Scene {
        match self {
            Scenes::DefaultScene(inner) => inner,
            Scenes::GameScene(inner) => inner,
        }
    }
}

pub struct DefaultScene;
impl Scene for DefaultScene {
    fn prepare(&self, ctx: &Context, state: &mut GameState, entity_map: &mut EntityMap) {
        let mut satelite = Satelite::new();

        let property = satelite.get_property();
        let physics = state.physical_world.register(property);
        satelite.register_physics(physics);

        entity_map.insert(ctx, satelite.typed());
    }

    fn tick(
        &self,
        _ctx: &Context,
        _state: &mut GameState,
        entity_map: &mut EntityMap,
    ) -> Option<SceneTickAction> {
        Some(SceneTickAction::ChangeScene(Scenes::GameScene(
            game::GameScene,
        )))
    }
}
