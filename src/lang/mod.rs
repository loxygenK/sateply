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
