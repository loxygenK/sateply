use std::sync::{Arc, Mutex};

use rlua::{prelude::*, Result as LuaResult, Table, Scope};

use super::{ClientError, ProgramClient};

pub type APIResult<T> = Result<T, APIError>;

#[derive(thiserror::Error, Debug)]
#[error("While performing {performing}: {error}")]
pub struct APIError {
    performing: String,
    error: ClientError,
}

impl APIError {
    pub fn new(performing: impl ToString, error: ClientError) -> Self {
        Self {
            performing: performing.to_string(),
            error,
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
    client
        .boost(&location, power)
        .map_err(|err| APIError::new("boosting", err))
}

pub fn register_api<'global, 'scope, T>(
    global: &Table<'global>,
    scope: &Scope<'global, 'scope>,
    client: &'scope mut T,
) -> LuaResult<()>
where
    T: ProgramClient + Send,
    'global: 'scope,
{
    let client = Arc::new(Mutex::new(client));

    let cloned_client = client.clone();
    global.set(
        "api_boost",
        scope.create_function(move |_, (location, power)| {
            // api_boost.lock().unwrap().boost(location, power).expect("TODO: panic message");
            boost(*cloned_client.lock().unwrap(), location, power).expect("");
            Ok(())
        })?,
    )?;

    let cloned_client = client.clone();
    global.set(
        "api_boost_2",
        scope.create_function(move |_, (location, power)| {
            // api_boost.lock().unwrap().boost(location, power).expect("TODO: panic message");
            boost(*cloned_client.lock().unwrap(), location, power).expect("");
            Ok(())
        })?,
    )?;

    Ok(())
}
