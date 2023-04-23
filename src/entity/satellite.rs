use std::{collections::HashMap, str::FromStr};

use ggez::{
    glam::Vec2,
    graphics::{self, Color},
    Context,
};

use super::{DrawInstruction, Entity, TypedEntity};
use crate::entity::RigidBody;
use crate::theory::geometry::Transform;
use crate::theory::physics::{PhysicsController, RigidBodyProperty};
use crate::{
    lang::{ClientError, ProgramClient},
    system::state::GameState,
    theory::physics::Physics,
};

#[derive(Debug)]
pub struct Satellite {
    pub physics: Option<Physics>,
    pub transform: Transform,
    pub booster: HashMap<SatelliteBoosters, f32>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SatelliteBoosters {
    BL,
    BR,
    FL,
    FR,
    WL,
    WR,
}

impl Satellite {
    pub fn new() -> Self {
        Self {
            physics: None,
            transform: Transform::default(),
            booster: HashMap::from([
                (SatelliteBoosters::BL, 0.0),
                (SatelliteBoosters::BR, 0.0),
                (SatelliteBoosters::FL, 0.0),
                (SatelliteBoosters::FR, 0.0),
                (SatelliteBoosters::WL, 0.0),
                (SatelliteBoosters::WR, 0.0),
            ]),
        }
    }
}

impl Default for Satellite {
    fn default() -> Self {
        Self::new()
    }
}

impl Entity for Satellite {
    fn update(&mut self, _ctx: &mut Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(
        &self,
        canvas: &mut graphics::Canvas,
        state: &GameState,
    ) -> ggez::GameResult<DrawInstruction> {
        canvas.draw(
            &state.satellite_svg,
            graphics::DrawParam::from(Vec2::new(0.0, 0.0))
                .color(Color::WHITE)
                .scale(Vec2::new(0.5, 0.5)),
        );

        Ok(DrawInstruction {
            position: self.transform.location.into(),
            angle: self.transform.angle,
            size: (
                (state.satellite_svg.width() as f32 / 2.0),
                (state.satellite_svg.height() as f32 / 2.0),
            )
                .into(),
            ..Default::default()
        })
    }

    fn typed(self) -> TypedEntity {
        TypedEntity::Satellite(self)
    }
}

impl RigidBody for Satellite {
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
        use SatelliteBoosters::*;

        fn relative(x: f32, y: f32) -> (f32, f32) {
            (141.0 / 2.0 * x, 48.0 / 2.0 * y)
        }

        let on = |location: SatelliteBoosters| -> f32 {
            *self.booster.get(&location).unwrap() * 250000.0
        };

        // BL
        controller.apply_force_locally(relative(-0.25, 0.0), (0.0, -on(BL)));

        // BR
        controller.apply_force_locally(relative(0.25, 0.0), (0.0, -on(BR)));

        // FL
        controller.apply_force_locally(relative(-0.25, 0.0), (0.0, on(FL)));

        // FR
        controller.apply_force_locally(relative(0.25, 0.0), (0.0, on(FR)));

        // WL
        controller.apply_force_locally(relative(-0.85, 0.0), (0.0, -on(WL)));

        // WR
        controller.apply_force_locally(relative(0.85, 0.0), (0.0, -on(WR)));
    }

    fn report_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
}

impl ProgramClient for Satellite {
    fn is_valid_booster(&self, name: &str) -> bool {
        name.parse::<SatelliteBoosters>().is_ok()
    }

    fn boost(&mut self, location: &str, power: f32) -> Result<(), ClientError> {
        let Ok(booster) = location.parse::<SatelliteBoosters>() else {
            return Err(ClientError::ValidationFailure{
                performing: "boosting".to_string(),
                part: "location".to_string(),
                reason: format!("Unknown booster ({location})"),
            });
        };

        if !(0.0..=1.0).contains(&power) {
            return Err(ClientError::ValidationFailure {
                performing: "boosting".to_string(),
                part: "power".to_string(),
                reason: "power should be in between 0 - 1".to_string(),
            });
        }

        self.booster.insert(booster, power);

        Ok(())
    }
}

impl FromStr for SatelliteBoosters {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BL" => Ok(Self::BL),
            "BR" => Ok(Self::BR),
            "FL" => Ok(Self::FL),
            "FR" => Ok(Self::FR),
            "WL" => Ok(Self::WL),
            "WR" => Ok(Self::WR),
            _ => Err(()),
        }
    }
}
