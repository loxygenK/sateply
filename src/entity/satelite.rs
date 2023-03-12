use ggez::{graphics::{self, Color, Rect}, glam::Vec2};

use crate::system::state::GameState;

use super::{Entity, TypedEntity};

#[derive(Debug)]
pub struct Satelite {
    pub x: f32,
    pub y: f32
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
    ) -> ggez::GameResult<Rect> {
        canvas.draw(
            &state.satelite_svg,
            graphics::DrawParam::from(Vec2::new(0.0, 0.0))
                .color(Color::WHITE)
                .scale(Vec2::new(0.5, 0.5))
        );

        Ok(Rect::new(self.x, self.y, state.satelite_svg.width() as f32, state.satelite_svg.height() as f32))
    }

    fn typed(self) -> TypedEntity {
        TypedEntity::Satelite(self)
    }
}

impl Satelite {
    pub fn advance(&mut self) {
        self.y -= 1.0;
    }
}
