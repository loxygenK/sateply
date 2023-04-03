use ggez::Context;

use crate::{extract_by_entity, utils::ExpectOnlyOneExt, system::state::GameState, lang::exec::execute};

use self::input::Control;

use super::Scene;

pub mod input;
pub mod satelite;

pub struct GameScene;
impl Scene for GameScene {
    fn prepare(&self, state: &mut GameState) {
        let satelite = extract_by_entity!(mut state.entities, Satelite)
            .unwrap_only_one();

        execute(
            satelite,
            r#"
            function main()
                api_boost("b", 0.5);
                return "";
            end
            "#
        ).unwrap();

        satelite.physics.transform.location = (640.0, 480.0).into();
        satelite.physics.transform.angle = 0.0;
    }

    fn tick(&self, ctx: &Context, state: &mut GameState) -> Option<super::SceneTickAction> {
        let enabled_control = Control::get_binding().get_active_controls(ctx);

        state.entities.update_all_entity().unwrap();
        None
    }
}
