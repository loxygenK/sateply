use ggez::{graphics, GameResult};

pub struct GameState {
    pub satellite_svg: graphics::Image,
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
        let satellite_svg = graphics::Image::from_path(ctx, "/imgs/satellite.png")?;

        Ok(Self {
            satellite_svg,
            next_lua_program: None,
        })
    }

    pub fn tick_state(&mut self) {}

    pub fn load_lua_program(&mut self, program: &str) {
        self.next_lua_program = Some(program.to_string());
    }
}
