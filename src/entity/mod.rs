use ggez::{glam::Vec2, graphics::Canvas, Context, GameResult};

use crate::entity::satellite::Satellite;

use crate::system::state::GameState;
use crate::theory::geometry::Transform;
use crate::theory::physics::{Physics, PhysicsController, RigidBodyProperty};

pub mod satellite;

pub trait Entity {
    fn update(&mut self, ctx: &mut Context) -> GameResult;
    fn draw(&self, canvas: &mut Canvas, state: &GameState) -> GameResult<DrawInstruction>;
    fn typed(self) -> TypedEntity;
}

pub trait RigidBody {
    fn get_property(&self) -> RigidBodyProperty;
    fn register_physics(&mut self, physics: Physics);
    fn get_mut_physics(&mut self) -> &mut Physics;
    fn update_physics(&mut self, controller: &mut PhysicsController);
    fn report_transform(&mut self, transform: Transform);
}

#[derive(Debug, Default)]
pub enum DrawOrigin {
    #[default]
    World,
    ScreenAbsolute,
}

#[derive(Default, Debug)]
pub struct DrawInstruction {
    pub position: Vec2,
    pub size: Vec2,
    pub angle: f32,
    pub draw_origin: DrawOrigin,
}

#[derive(Debug)]
pub enum TypedEntity {
    Satellite(Satellite),
}

impl TypedEntity {
    pub fn inner(&self) -> &dyn Entity {
        match self {
            TypedEntity::Satellite(inner) => inner,
        }
    }

    pub fn inner_mut(&mut self) -> &mut dyn Entity {
        match self {
            TypedEntity::Satellite(inner) => inner,
        }
    }

    pub fn as_mut_rigidbody(&mut self) -> Option<&mut dyn RigidBody> {
        match self {
            TypedEntity::Satellite(inner) => Some(inner),
        }
    }
}
