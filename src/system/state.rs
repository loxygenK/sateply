use ggez::graphics::Canvas;
use ggez::{graphics, GameResult};
use std::collections::HashMap;

use crate::world::{World, WorldKey};
use crate::theory::physics::PhysicalWorld;

pub struct GameState {
    pub satelite_svg: graphics::Image,
    pub next_lua_program: Option<String>,
}

#[derive(PartialEq, Eq)]
pub enum KeyPressTiming {
    Pressed { repeated: bool },
    Pressing,
    Released,
}

impl GameState {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<Self> {
        let satelite_svg = graphics::Image::from_path(ctx, "/imgs/satellite.png")?;

        Ok(Self {
            satelite_svg,
            next_lua_program: None
        })
    }

    pub fn tick_state(&mut self) {}

    pub fn load_lua_program(&mut self, program: &str) {
        self.next_lua_program = Some(program.to_string());
    }
}
