use ggez::{input::keyboard::KeyInput, Context};

use crate::world::World;
use crate::entity::RigidBody;
use crate::{
    entity::Entity,
    extract_by_entity,
    system::{keyinput_list::KeyTypeMatchMap, state::GameState},
    theory::physics::Physics,
    traitext::ExpectOnlyOneExt,
};

pub mod game;

pub trait Scene {
    fn prepare(&mut self, ctx: &Context, state: &mut GameState, entity_map: &mut World);
    fn tick(
        &mut self,
        ctx: &mut Context,
        state: &mut GameState,
        entity_map: &mut World,
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

    pub fn inner_mut(&mut self) -> &mut dyn Scene {
        match self {
            Scenes::DefaultScene(inner) => inner,
            Scenes::GameScene(inner) => inner,
        }
    }
}

pub struct DefaultScene;
impl Scene for DefaultScene {
    fn prepare(&mut self, ctx: &Context, state: &mut GameState, entity_map: &mut World) {
        // let mut satelite = Satellite::new();

        // let property = satelite.get_property();
        // let physics = state.physical_world.register(property);
        // satelite.register_physics(physics);

        // entity_map.insert(ctx, satelite.typed());
    }

    fn tick(
        &mut self,
        _ctx: &mut Context,
        _state: &mut GameState,
        entity_map: &mut World,
    ) -> Option<SceneTickAction> {
        // Some(SceneTickAction::ChangeScene(Scenes::GameScene(
        //     game::GameScene::new(),
        // )))

        None
    }
}
