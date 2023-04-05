use ggez::Context;

use crate::{extract_by_entity, utils::ExpectOnlyOneExt, system::state::GameState, lang::exec::execute};
use crate::entity::map::EntityMap;

use self::input::Control;

use super::Scene;

pub mod input;
pub mod satelite;

pub struct GameScene;
impl Scene for GameScene {
    fn prepare(&self, _ctx: &Context, _state: &mut GameState, entity_map: &mut EntityMap) {
        // do nothing
    }

    fn tick(&self, ctx: &Context, state: &mut GameState, entity_map: &mut EntityMap) -> Option<super::SceneTickAction> {
        let enabled_control = Control::get_binding().get_active_controls(ctx);

        entity_map.update_all_entity(&mut state.physical_world).unwrap();
        None
    }
}
