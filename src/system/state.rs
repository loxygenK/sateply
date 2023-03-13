use ggez::{graphics, GameResult, input::keyboard::{KeyCode, KeyInput}};

use crate::entity::map::EntityMap;

pub struct GameState {
    pub entities: EntityMap,
    pub pressed_key: Vec<(KeyInput, KeyPressTiming)>,
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
                pressed_key: Vec::new(),
                satelite_svg,
            }
        )
    }

    pub fn tick_state(&mut self) {
        self.pressed_key.retain(|(_, timing)| timing != &KeyPressTiming::Pressed { repeated: true });
        self.pressed_key.iter_mut()
            .filter(|(_, timing)| timing == &KeyPressTiming::Pressed { repeated: false })
            .for_each(|key| key.1 = KeyPressTiming::Pressing);
    }
}
