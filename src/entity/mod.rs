use ggez::GameResult;

pub mod satelite;

pub trait Entity {
    fn update(&mut self) -> GameResult;
    fn draw(&mut self) -> GameResult;
}
