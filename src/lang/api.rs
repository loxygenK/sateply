use std::sync::{Arc, Mutex};

use rlua::{prelude::*, Result as LuaResult, Table, Scope};

use super::{ClientError, ProgramClient, ProgramEnvironment};

use crate::lang::ModKey;

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

pub fn boost<T: ProgramClient>(client: &mut T, location: String, power: f32) -> APIResult<()> {
    client
        .boost(&location, power)
        .map_err(|err| APIError::new("boost", err))
}

pub fn is_pressed<T: ProgramEnvironment>(env: &T, char: String, mods: u8) -> APIResult<bool> {
    env
        .is_pressed(&char, ModKey::from_bits(mods).unwrap_or(ModKey::empty()))
        .map_err(|err| APIError::new("is_pressed", err))
}

pub fn register_api<'global, 'scope, T, E>(
    global: &Table<'global>,
    scope: &Scope<'global, 'scope>,
    client: &'scope mut T,
    env: &'scope E,
) -> LuaResult<()>
where
    T: ProgramClient + Send,
    E: ProgramEnvironment + Send,
    'global: 'scope,
{
    let client = Arc::new(Mutex::new(client));
    let env = Arc::new(Mutex::new(env));

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
    let cloned_scope = scope.clone();
    global.set(
        "api_is_pressed",
        scope.create_function(move |_, (key)| {
            Ok(is_pressed(*env.lock().unwrap(), key, 0).expect(""))
        })?,
    )?;

    Ok(())
}
