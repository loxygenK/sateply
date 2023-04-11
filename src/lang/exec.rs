use std::sync::{Arc, Mutex};

use rlua::{prelude::*, StdLib};
use rlua::{Error, Function, Result as LuaResult};

use crate::lang::api::boost;

use super::api::register_api;
use super::{ProgramClient, ProgramEnvironment};

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum ExecutionError {
    #[error("SyntaxError: {0}")]
    SyntaxError(String),

    #[error("ProgrammaticError: {0}")]
    ProgrammaticError(String),

    #[error("Problem occurred with the runtime: {0}")]
    EnvironmentalError(String),

    #[error("DynamicError: {0}")]
    DynamicError(String),

    #[error("The program reported error when finishing execution: {0}")]
    Reported(String),

    #[error("Firmware does not have function 'main'")]
    EntrypointNotFound,

    #[error("Firmware must return String")]
    InvalidEntrypointReturnType,
}

pub struct LuaProgramExecutor {
    runtime: Lua
}

impl LuaProgramExecutor {
    pub fn new() -> Self {
        Self {
            runtime: Lua::new_with(StdLib::BASE)
        }
    }

    pub fn load(&mut self, program: &str) -> Result<(), ExecutionError> {
        self.runtime.context(|ctx| {
            let global = ctx.globals();

            ctx.load(program).eval::<()>().map_err(map_execute_result)?;

            if !global.contains_key("main").map_err(map_execute_result)? {
                return Err(ExecutionError::EntrypointNotFound);
            }

            Ok(())
        })
    }

    pub fn execute<C, E>(&mut self, client: &mut C, env: &E) -> Result<(), ExecutionError>
        where C: ProgramClient + Send,
              E: ProgramEnvironment + Send,
    {
        let reported = self.runtime.context(|ctx| {
            let global = ctx.globals();
            let main: Function = global.get("main").map_err(map_execute_result)?;

            ctx.scope(|scope| {
                register_api(&global, scope, client).map_err(map_execute_result)?;

                let reported: String = main.call("").map_err(map_execute_result)?;

                Ok(reported)
            })
        })?;

        if reported.is_empty() {
            Ok(())
        } else {
            Err(ExecutionError::Reported(reported.to_string()))
        }
    }
}

fn map_execute_result(error: LuaError) -> ExecutionError {
    #[allow(unreachable_patterns)]
    match error {
        Error::SyntaxError { message, .. } => ExecutionError::SyntaxError(message),
        Error::RuntimeError(msg) => ExecutionError::DynamicError(msg),
        Error::MemoryError(msg) => ExecutionError::EnvironmentalError(msg),
        Error::RecursiveMutCallback => {
            ExecutionError::ProgrammaticError("Mutable callback ran twice".to_string())
        }
        Error::CallbackDestructed => {
            ExecutionError::EnvironmentalError("Callback is destructed".to_string())
        }
        Error::StackError => {
            ExecutionError::EnvironmentalError("No more space to place stack!".to_string())
        }
        Error::BindError => {
            ExecutionError::EnvironmentalError("Too many arguments to bind".to_string())
        }
        Error::ToLuaConversionError { from, to, message } => ExecutionError::DynamicError(format!(
            "Cannot convert the value from the runtime to the firmware(lua) ('{}' => '{}'): {}",
            from,
            to,
            message.unwrap_or("Error message not present".to_string())
        )),
        Error::FromLuaConversionError { from, to, message } => {
            ExecutionError::DynamicError(format!(
                "Cannot convert the value from the firmware(lua) to the runtime ('{}' => '{}'): {}",
                from,
                to,
                message.unwrap_or("Error message not present".to_string())
            ))
        }
        Error::CoroutineInactive => {
            ExecutionError::EnvironmentalError("should be unreachable".to_string())
        }

        // These errors seem to be occur when the Rust side using `AnyUserData` and
        // somehow the lua (?) program violated the rust's important rule
        Error::UserDataTypeMismatch => {
            ExecutionError::EnvironmentalError("should be unreachable".to_string())
        }
        Error::UserDataBorrowError => {
            ExecutionError::EnvironmentalError("should be unreachable".to_string())
        }
        Error::UserDataBorrowMutError => {
            ExecutionError::EnvironmentalError("should be unreachable".to_string())
        }

        Error::MismatchedRegistryKey => ExecutionError::EnvironmentalError(
            "should be unreachable (state contaminated)".to_string(),
        ),
        Error::CallbackError { traceback, cause } => {
            ExecutionError::DynamicError(format!("API call has failed: {cause:#?}\n{traceback}"))
        }
        Error::ExternalError(cause) => ExecutionError::DynamicError(format!(
            "API call has failed due to the external cause: {cause}"
        )),
        _ => ExecutionError::EnvironmentalError("Unknown error occurred!!".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::lang::{ClientError, ModKey};

    use super::*;

    #[derive(Default)]
    pub struct Client {
        booster: HashMap<String, f32>,
    }
    impl ProgramClient for Client {
        fn is_valid_booster(&self, name: &str) -> bool {
            name.starts_with("booster_")
        }

        fn boost(&mut self, location: &str, power: f32) -> Result<(), ClientError> {
            if !self.is_valid_booster(location) {
                return Err(ClientError::ValidationFailure {
                    performing: "boost".to_owned(),
                    part: "location".to_owned(),
                    reason: "Booster name is not valid".to_owned(),
                });
            }

            if !(0.0..=1.0).contains(&power) {
                return Err(ClientError::ValidationFailure {
                    performing: "boost".to_owned(),
                    part: "power".to_owned(),
                    reason: "Booster output is not valid".to_owned(),
                });
            }

            self.booster.insert(location.to_string(), power);
            Ok(())
        }
    }

    struct Environment;
    impl ProgramEnvironment for Environment {
        fn is_pressed(&self, char: &str, mods: ModKey) -> Option<bool> {
            if char.len() != 1 {
                return None;
            }

            let char: char = char.chars().nth(0).unwrap();
            Some(char.is_alphabetic())
        }
    }

    #[test]
    fn runtime_should_success_if_empty_string_is_returned() {
        let mut executor = LuaProgramExecutor::new();
        executor.load("function main() return '' end").unwrap();
        let result = executor.execute(&mut Client::default(), &Environment);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn runtime_should_fail_if_non_empty_string_is_returned() {
        let mut executor = LuaProgramExecutor::new();

        let seed = "(This is a message)";
        executor.load(&format!("function main() return '{seed}' end"));

        let result = executor.execute(&mut Client::default(), &Environment);

        assert_eq!(result, Err(ExecutionError::Reported(seed.to_string())));
    }

    #[test]
    fn runtime_should_able_to_call_rusty_client_from_lua() {
        let mut executor = LuaProgramExecutor::new();
        let mut client = Client::default();

        assert!(!client.booster.contains_key("booster_A"));

        executor.load(
            r#"
            function main()
                api_boost("booster_A", 0.5);
                api_boost_2("booster_B", 0.3);
                return ''
            end
            "#,
        );
        let result = executor.execute(&mut client, &Environment);

        assert_eq!(result, Ok(()));
        assert_eq!(client.booster.get("booster_A"), Some(&0.5));
        assert_eq!(client.booster.get("booster_B"), Some(&0.3));
    }
}
