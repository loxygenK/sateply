use ggez::{graphics::{self, Color, Rect, DrawParam, Text}, glam::Vec2};

use crate::{system::{state::GameState}, theory::physics::Physics};

use super::{Entity, TypedEntity, DrawInstruction};

#[derive(Debug)]
pub struct Satelite {
    pub physics: Physics
}

impl Satelite {
    pub fn new() -> Self {
        let mut physics = Physics::default();
        physics.apply_force((0.0, -0.001).into());
        physics.apply_angular_force(0.001);

        Self { physics }
    }
}

impl Entity for Satelite {
    fn update(&mut self) -> ggez::GameResult {
        self.physics.tick();

        Ok(())
    }

    fn draw(
        &self,
        canvas: &mut graphics::Canvas,
        state: &GameState
    ) -> ggez::GameResult<DrawInstruction> {
        canvas.draw(
            &state.satelite_svg,
            graphics::DrawParam::from(Vec2::new(0.0, 0.0))
                .color(Color::WHITE)
                .scale(Vec2::new(0.5, 0.5))
        );

        Ok(DrawInstruction {
            position: self.physics.transform.location.into(),
            angle: self.physics.transform.angle,
            size: ((state.satelite_svg.width() as f32 / 2.0), (state.satelite_svg.height() as f32 / 2.0)).into(),
        })
    }

    fn typed(self) -> TypedEntity {
        TypedEntity::Satelite(self)
    }
}
