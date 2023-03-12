use crate::{system::GameState, extract_by_entity, utils::ExpectOnlyOneExt, entity::Entity};

pub trait Scene {
    fn prepare(&self, state: &mut GameState);
    fn tick(&self, state: &mut GameState);
}

pub enum Scenes {
    DefaultScene(DefaultScene)
}

impl Scenes {
    pub fn inner(&self) -> &impl Scene {
        match self {
            Scenes::DefaultScene(inner) => inner
        }
    }
}

pub struct DefaultScene;
impl Scene for DefaultScene {
    fn prepare(&self, state: &mut GameState) {
        let mut satelite = extract_by_entity!(mut state.entities, Satelite)
            .unwrap_only_one();

        satelite.x = 640;
        satelite.y = 480;
    }

    fn tick(&self, state: &mut GameState) {
        state.entities
            .iter_mut_entity()
            .for_each(|entity| {
                entity.inner_mut().update().expect("Update should be success");
            });
    }
}
