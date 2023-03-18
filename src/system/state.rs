use ggez::{graphics, GameResult};

use crate::entity::map::EntityMap;

pub struct GameState {
    pub entities: EntityMap,
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
                satelite_svg,
            }
        )
    }

    pub fn tick_state(&mut self) {
    }
}
