use std::collections::HashMap;

use ggez::{GameResult, graphics::{Canvas, Rect}, glam::Vec2};
use rand::{thread_rng, RngCore};

use crate::system::state::GameState;

use self::satelite::Satelite;

pub mod satelite;
pub mod map;

pub trait Entity {
    fn update(&mut self) -> GameResult;
    fn draw(&self, canvas: &mut Canvas, state: &GameState) -> GameResult<DrawInstruction>;
    fn typed(self) -> TypedEntity;
}

#[derive(Debug)]
pub struct DrawInstruction {
    pub position: Vec2,
    pub size: Vec2,
    pub angle: f32,
}

#[derive(Debug)]
pub enum TypedEntity {
    Satelite(Satelite)
}

impl TypedEntity {
    pub fn inner(&self) -> &impl Entity {
        match self {
            TypedEntity::Satelite(inner) => inner
        }
    }

    pub fn inner_mut(&mut self) -> &mut impl Entity {
        match self {
            TypedEntity::Satelite(inner) => inner
        }
    }
}
