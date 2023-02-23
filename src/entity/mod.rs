use ggez::{GameResult, graphics::Canvas, glam::Vec2};

use crate::GameState;

pub mod satelite;

pub trait Entity {
    fn update(&mut self) -> GameResult;
    fn draw(&self, canvas: &mut Canvas, state: &GameState) -> GameResult<Vec2>;
}
