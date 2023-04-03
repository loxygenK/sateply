use super::{ProgramClient, ClientError};

pub type APIResult<T> = Result<T, APIError>;

#[derive(thiserror::Error, Debug)]
#[error("While performing {performing}: {error}")]
pub struct APIError {
    performing: String,
    error: ClientError
}

impl APIError {
    pub fn new(performing: impl ToString, error: ClientError) -> Self {
        Self {
            performing: performing.to_string(),
            error
        }
    }
}

pub struct API<'client, T: ProgramClient>(pub &'client mut T);
impl<'client, T: ProgramClient> API<'client, T> {
    pub fn boost(&mut self, location: String, power: f32) -> APIResult<()> {
        boost(self.0, location, power)
    }
}

pub fn boost<T: ProgramClient>(client: &mut T, location: String, power: f32) -> APIResult<()> {
    client.boost(&location, power).map_err(|err| APIError::new("boosting", err))
}