use ggez::Context;

use crate::{extract_by_entity, utils::ExpectOnlyOneExt, system::state::GameState, lang::exec::execute};

use self::input::Control;

use super::Scene;

pub mod input;
pub mod satelite;

pub struct GameScene;
impl Scene for GameScene {
    fn prepare(&self, _state: &mut GameState) {
        // do nothing
    }

    fn tick(&self, ctx: &Context, state: &mut GameState) -> Option<super::SceneTickAction> {
        let enabled_control = Control::get_binding().get_active_controls(ctx);

        state.entities.update_all_entity(&mut state.physical_world).unwrap();
        None
    }
}
