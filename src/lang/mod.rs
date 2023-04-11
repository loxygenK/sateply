pub mod api;
pub mod exec;

#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("Validation failure, '{part}': {reason}")]
    ValidationFailure {
        performing: String,
        part: String,
        reason: String,
    },
}

pub trait ProgramClient {
    fn is_valid_booster(&self, name: &str) -> bool;
    fn boost(&mut self, location: &str, power: f32) -> Result<(), ClientError>;
}

bitflags::bitflags! {
    pub struct ModKey: u8 {
        const NONE   = 0b0000000;
        const LSHIFT = 0b0000001;
        const LCTRL  = 0b0000010;
        const LALT   = 0b0000100;
        const RSHIFT = 0b0001000;
        const RCTRL  = 0b0010000;
        const RALT   = 0b0100000;
        const SHIFT  = 0b0001001;
        const CTRL   = 0b0010010;
        const ALT    = 0b0100100;
    }
}

pub trait ProgramEnvironment {
    fn is_pressed(&self, char: &str, mods: ModKey) -> Option<bool>;
}
