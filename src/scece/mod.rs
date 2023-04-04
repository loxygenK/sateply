use ggez::{input::keyboard::KeyInput, Context};

use crate::{system::{state::GameState, keyinput_list::KeyTypeMatchMap}, extract_by_entity, utils::ExpectOnlyOneExt, entity::{Entity, satelite::Satelite}, theory::physics::Physics};
use crate::entity::RigidBody;

pub mod game;

pub trait Scene {
    fn prepare(&self, state: &mut GameState);
    fn tick(&self, ctx: &Context, state: &mut GameState) -> Option<SceneTickAction>;
}

pub enum SceneTickAction {
    ChangeScene(Scenes)
}

pub enum Scenes {
    DefaultScene(DefaultScene),
    GameScene(game::GameScene),
}

impl Scenes {
    pub fn inner(&self) -> &dyn Scene {
        match self {
            Scenes::DefaultScene(inner) => inner,
            Scenes::GameScene(inner) => inner
        }
    }
}

pub struct DefaultScene;
impl Scene for DefaultScene {
    fn prepare(&self, state: &mut GameState) {
        let mut satelite = Satelite::new();

        let property = satelite.get_property();
        let physics = state.physical_world.register(property);
        satelite.register_physics(physics);

        state.entities.insert(satelite.typed());
    }

    fn tick(&self, _ctx: &Context, _state: &mut GameState) -> Option<SceneTickAction> {
        Some(SceneTickAction::ChangeScene(
            Scenes::GameScene(game::GameScene)
        ))
    }
}
