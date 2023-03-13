use crate::{system::{state::GameState}, extract_by_entity, utils::ExpectOnlyOneExt, entity::{Entity, satelite::Satelite}, theory::physics::Physics};

pub mod game;

pub trait Scene {
    fn prepare(&self, state: &mut GameState);
    fn tick(&self, state: &mut GameState) -> Option<SceneTickAction>;
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
        state.entities.insert(Satelite::new().typed());
    }

    fn tick(&self, _state: &mut GameState) -> Option<SceneTickAction> {
        Some(SceneTickAction::ChangeScene(
            Scenes::GameScene(game::GameScene)
        ))
    }
}
