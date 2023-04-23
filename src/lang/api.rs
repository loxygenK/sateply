use std::sync::{Arc, Mutex};

use rlua::{Result as LuaResult, Scope, Table};

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
    env.is_pressed(&char, ModKey::from_bits(mods).unwrap_or(ModKey::empty()))
        .map_err(|err| APIError::new("is_pressed", err))
}

pub fn register_api<'global, 'scope, T, E>(
    api_table: &Table<'global>,
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

    macro_rules! register {
        ($name: ident(client, env, $( $arg: ident ),+)) => {
            let cloned_client = client.clone();
            let cloned_env = env.clone();
            api_table.set(
                stringify!($name),
                scope.create_function(move |_, ($( $arg ),+)| {
                    Ok($name(
                        *cloned_client.lock().unwrap(),
                        *cloned_env.lock().unwrap(),
                        $( $arg ),+
                    ).unwrap())
                })?,
            )?;
        };
        ($name: ident(client, $( $arg: ident ),+)) => {
            let cloned_client = client.clone();
            api_table.set(
                stringify!($name),
                scope.create_function(move |_, ($( $arg ),+)| {
                    Ok($name(
                        *cloned_client.lock().unwrap(),
                        $( $arg ),+
                    ).unwrap())
                })?,
            )?;
        };
        ($name: ident(env, $( $arg: ident ),+)) => {
            let cloned_env = env.clone();
            api_table.set(
                stringify!($name),
                scope.create_function(move |_, ($( $arg ),+)| {
                    Ok($name(
                        *cloned_env.lock().unwrap(),
                        $( $arg ),+
                    ).unwrap())
                })?,
            )?;
        };
        ($name: ident($( $arg: ident ),+)) => {
            api_table.set(
                stringify!($name),
                scope.create_function(move |_, ($( $arg ),+)| {
                    Ok($name($( $arg ),+).unwrap())
                })?,
            )?;
        };
    }

    register!(boost(client, location, power));
    register!(is_pressed(env, location, power));

    Ok(())
}
