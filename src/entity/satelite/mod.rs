use std::{collections::HashMap, str::FromStr};

use ggez::{graphics::{self, Color}, glam::{Vec2, vec2}, Context};

use crate::{system::state::GameState, theory::{physics::Physics, geometry::Vector}, lang::{ProgramClient, ClientError}};
use crate::entity::RigidBody;
use crate::theory::geometry::Transform;
use crate::theory::physics::{PhysicsController, RigidBodyProperty};
use super::{Entity, TypedEntity, DrawInstruction};

#[derive(Debug)]
pub struct Satelite {
    pub physics: Option<Physics>,
    pub transform: Transform,
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
            physics: None,
            transform: Transform::default(),
            booster: HashMap::from([
                (SateliteBoosters::Front, 0.0),
                (SateliteBoosters::Back, 0.0),
            ])
        }
    }
}

impl Entity for Satelite {
    fn update(&mut self) -> ggez::GameResult {
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
            position: self.transform.location.into(),
            angle: self.transform.angle,
            size: ((state.satelite_svg.width() as f32 / 2.0), (state.satelite_svg.height() as f32 / 2.0)).into(),
        })
    }

    fn typed(self) -> TypedEntity {
        TypedEntity::Satelite(self)
    }
}

impl RigidBody for Satelite {
    fn get_property(&self) -> RigidBodyProperty {
        RigidBodyProperty {
            mass: 1000.0,
            size: (141.0, 48.0),
            initial_transform: self.transform.clone(),
        }
    }

    fn register_physics(&mut self, physics: Physics) {
        self.physics = Some(physics);
    }

    fn get_mut_physics(&mut self) -> &mut Physics {
        self.physics.as_mut().unwrap()
    }

    fn update_physics(&mut self, controller: &mut PhysicsController) {
        controller.apply_force_locally(
            (-141.0 / 2.0, 0.0),// (-141.0 / 2.0, -48.0 / 2.0),
            (0.0, 10000.0)
        );

        controller.apply_force_locally(
            (141.0 / 2.0, 0.0),// (-141.0 / 2.0, -48.0 / 2.0),
            (0.0, -10000.0)
        );
    }

    fn report_transform(&mut self, transform: Transform) {
        self.transform = transform;
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

        if !(0.0..=1.0).contains(&power) {
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
