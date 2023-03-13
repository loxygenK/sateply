use crate::{extract_by_entity, utils::ExpectOnlyOneExt, system::state::GameState};

use super::Scene;

pub struct GameScene;
impl Scene for GameScene {
    fn prepare(&self, state: &mut GameState) {
        let satelite = extract_by_entity!(mut state.entities, Satelite)
            .unwrap_only_one();

        satelite.physics.transform.location = (640.0, 480.0).into();
        satelite.physics.transform.angle = 0.0;
    }

    fn tick(&self, state: &mut GameState) -> Option<super::SceneTickAction> {
        state.entities.update_all_entity().unwrap();

        None
    }
}
