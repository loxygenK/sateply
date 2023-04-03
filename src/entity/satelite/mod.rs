use std::{collections::HashMap, str::FromStr};

use ggez::{graphics::{self, Color}, glam::Vec2};

use crate::{system::state::GameState, theory::physics::Physics, lang::{ProgramClient, ClientError}};
use super::{Entity, TypedEntity, DrawInstruction};

#[derive(Debug)]
pub struct Satelite {
    pub physics: Physics,
    pub booster: HashMap<SateliteBoosters, f32>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SateliteBoosters {
    Front,
    Back
}

impl Satelite {
    pub fn new() -> Self {
        Self {
            physics: Physics::default(),
            booster: HashMap::from([
                (SateliteBoosters::Front, 0.0),
                (SateliteBoosters::Back, 0.0),
            ])
        }
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

impl ProgramClient for Satelite {
    fn is_valid_booster(&self, name: &str) -> bool {
        name.parse::<SateliteBoosters>().is_ok()
    }

    fn boost(&mut self, location: &str, power: f32) -> Result<(), ClientError> {
        let Ok(booster) = location.parse::<SateliteBoosters>() else {
            return Err(ClientError::ValidationFailure{
                performing: "boosting".to_string(),
                part: "location".to_string(),
                reason: "power should be in between 0 - 1".to_string()
            });
        };

        if (0.0..=1.0).contains(&power) {
            return Err(ClientError::ValidationFailure{
                performing: "boosting".to_string(),
                part: "power".to_string(),
                reason: "power should be in between 0 - 1".to_string()
            });
        }

        self.booster.insert(booster, power);
        Ok(())
    }
}

impl FromStr for SateliteBoosters {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "f" | "front" => Ok(Self::Front),
            "b" | "back" => Ok(Self::Back),
            _ => Err(())
        }
    }
}
