use ggez::{graphics, GameResult};

use crate::entity::map::EntityMap;
use crate::theory::physics::PhysicalWorld;

pub struct GameState {
    pub entities: EntityMap,
    pub physical_world: PhysicalWorld,
    pub satelite_svg: graphics::Image,
}

#[derive(PartialEq, Eq)]
pub enum KeyPressTiming {
    Pressed { repeated: bool },
    Pressing,
    Released
}

impl GameState {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<Self> {
        let satelite_svg = graphics::Image::from_path(ctx, "/imgs/satelite.png")?;

        Ok(
            Self {
                entities: EntityMap::default(),
                physical_world: PhysicalWorld::new(),
                satelite_svg,
            }
        )
    }

    pub fn tick_state(&mut self) {
    }
}
