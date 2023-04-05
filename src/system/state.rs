use ggez::graphics::Canvas;
use ggez::{graphics, GameResult};
use std::collections::HashMap;

use crate::entity::map::{EntityMap, EntityMapKey};
use crate::theory::physics::PhysicalWorld;

pub struct GameState {
    pub physical_world: PhysicalWorld,
    pub satelite_svg: graphics::Image,
}

#[derive(PartialEq, Eq)]
pub enum KeyPressTiming {
    Pressed { repeated: bool },
    Pressing,
    Released,
}

impl GameState {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<Self> {
        let satelite_svg = graphics::Image::from_path(ctx, "/imgs/satelite.png")?;

        Ok(Self {
            physical_world: PhysicalWorld::new(),
            satelite_svg,
        })
    }

    pub fn tick_state(&mut self) {}
}
