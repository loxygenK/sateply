use ggez::Context;

use crate::entity::TypedEntity;
use crate::world::World;
use crate::lang::exec::{LuaProgramExecutor, ExecutionError};
use crate::{
    extract_by_entity, system::state::GameState, traitext::ExpectOnlyOneExt,
};

use self::{lang_env::Environment, input::Control};

use super::Scene;

pub mod input;
pub mod lang_env;
pub mod satelite;

pub struct GameScene {
    executor: LuaProgramExecutor,
    execute_by_frame: bool,
}
impl Scene for GameScene {
    fn prepare(&mut self, _ctx: &Context, state: &mut GameState, entity_map: &mut World) {
    }

    fn tick(
        &mut self,
        ctx: &mut Context,
        state: &mut GameState,
        entity_map: &mut World,
    ) -> Option<super::SceneTickAction> {

        if self.execute_by_frame {
            self.execute(ctx, entity_map);
        }

        None
    }
}
impl GameScene {
    pub fn new() -> Self {
        GameScene {
            executor: LuaProgramExecutor::new(),
            execute_by_frame: true,
        }
    }

    pub fn load_program(&mut self, program: &str) -> Result<(), ExecutionError> {
        Ok(())
    }

    pub fn start_frame_execution(&mut self) {
        self.execute_by_frame = true;
    }

    pub fn stop_frame_execution(&mut self) {
        self.execute_by_frame = false;
    }

    pub fn execute(&mut self, ctx: &Context, entity_map: &mut World) {
    }
}
