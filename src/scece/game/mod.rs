use ggez::Context;

use crate::entity::TypedEntity;
use crate::entity::map::EntityMap;
use crate::lang::exec::{LuaProgramExecutor, ExecutionError};
use crate::{
    extract_by_entity, system::state::GameState, utils::ExpectOnlyOneExt,
};

use self::{lang_env::Environment, input::Control};

use super::Scene;

pub mod input;
mod lang_env;
pub mod satelite;

pub struct GameScene {
    executor: LuaProgramExecutor,
    execute_by_frame: bool,
}
impl Scene for GameScene {
    fn prepare(&mut self, _ctx: &Context, state: &mut GameState, entity_map: &mut EntityMap) {
    }

    fn tick(
        &mut self,
        ctx: &mut Context,
        state: &mut GameState,
        entity_map: &mut EntityMap,
    ) -> Option<super::SceneTickAction> {
        entity_map
            .update_all_entity(ctx, &mut state.physical_world)
            .unwrap();

        if let Some(program) = &state.next_lua_program {
            self.executor.load(&program);
            state.next_lua_program = None;
        }

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
        self.executor.load(program)
    }

    pub fn start_frame_execution(&mut self) {
        self.execute_by_frame = true;
    }

    pub fn stop_frame_execution(&mut self) {
        self.execute_by_frame = false;
    }

    pub fn execute(&mut self, ctx: &Context, entity_map: &mut EntityMap) {
        let mut satelite = extract_by_entity!(mut entity_map, Satelite)
            .unwrap_only_one();

        let result = self.executor.execute(satelite, &Environment::new(&ctx.keyboard));

        #[cfg(debug_assertions)]
        if let Err(err) = result {
            println!("{err}");
        }
    }
}
