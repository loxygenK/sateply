use ggez::{graphics::{self, Color}, glam::Vec2};

use crate::GameState;

use super::{Entity, TypedEntity};

pub struct Satelite {
    pub x: usize,
    pub y: usize
}

impl Entity for Satelite {
    fn update(&mut self) -> ggez::GameResult {
        self.advance();

        Ok(())
    }

    fn draw(
        &self,
        canvas: &mut graphics::Canvas,
        state: &GameState
    ) -> ggez::GameResult<Vec2> {
        canvas.draw(
            &state.satelite_svg,
            graphics::DrawParam::from(Vec2::new(0.0, 0.0))
                .color(Color::WHITE)
                .scale(Vec2::new(0.5, 0.5))
        );

        Ok((640.0, 480.0).into())
    }

    fn typed(self) -> TypedEntity {
        TypedEntity::Satelite(self)
    }
}

impl Satelite {
    pub fn advance(&mut self) {
        self.y -= 1;
    }
}
