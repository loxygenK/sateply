use std::sync::{Arc, Mutex};

use hlua::{Lua, LuaError, AnyLuaValue, Function, LuaFunction};
use crate::{lang::api::API, entity::map};

use super::ProgramClient;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum ExecutionError {
    #[error("SyntaxError: {0}")]
    SyntaxError(String),

    #[error("ProgrammaticError: {0}")]
    ProgrammaticError(String),

    #[error("DynamicError: {0}")]
    DynamicError(String),

    #[error("The program reported error when finishing execution: {0}")]
    Reported(String),

    #[error("Firmware does not have function 'main'")]
    EntrypointNotFound,

    #[error("Firmware must return String")]
    InvalidEntrypointReturnType
}

pub fn register_api<'lua, 'client: 'lua>(lua: &mut Lua<'lua>, client: &'client mut impl ProgramClient) {
    let api = API { client };
    let api = Arc::new(Mutex::new(api));

    let api_boost = api.clone();
    lua.set(
        "api_boost",
        hlua::function2(move |location, power| api_boost.lock().unwrap().boost(location,power).map_err(|err| format!("{}", err)))
    );
}

pub fn execute(client: &mut impl ProgramClient, code: &str) -> Result<(), ExecutionError> {
    let mut lua_runtime = Lua::new();

    register_api(&mut lua_runtime, client);

    lua_runtime.open_base();
    lua_runtime.execute(code).map_err(map_execute_result)?;

    let Some(mut main) = lua_runtime.get::<LuaFunction<_>, _>("main") else {
        return Err(ExecutionError::EntrypointNotFound);
    };

    let reported = main.call::<String>().map_err(map_execute_result)?;

    if reported.is_empty() {
        Ok(())
    } else {
        Err(ExecutionError::Reported(reported.to_string()))
    }
}

fn map_execute_result(error: LuaError) -> ExecutionError {
    match error {
        LuaError::SyntaxError(err) => ExecutionError::SyntaxError(err),
        LuaError::ExecutionError(err) => ExecutionError::ProgrammaticError(err),
        LuaError::WrongType => ExecutionError::InvalidEntrypointReturnType,
        LuaError::ReadError(err) => ExecutionError::DynamicError(err.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::lang::ClientError;

    use super::*;

    #[derive(Default)]
    pub struct Client {
        booster: HashMap<String, f32>
    }
    impl ProgramClient for Client {
        fn is_valid_booster(&self, name: &str) -> bool {
            name.starts_with("booster_")
        }

        fn boost(&mut self, location: &str, power: f32) -> Result<(), crate::lang::ClientError> {
            if !self.is_valid_booster(location) {
                return Err(ClientError::ValidationFailure {
                    performing: "boost".to_owned(),
                    part: "location".to_owned(),
                    reason: "Booster name is not valid".to_owned()
                })
            }

            if !(0.0..=1.0).contains(&power) {
                return Err(ClientError::ValidationFailure {
                    performing: "boost".to_owned(),
                    part: "power".to_owned(),
                    reason: "Booster output is not valid".to_owned()
                })
            }

            self.booster.insert(location.to_string(), power);
            Ok(())
        }
    }

    #[test]
    fn runtime_should_success_if_empty_string_is_returned() {
        let result = execute(&mut Client::default(), "function main() return '' end");

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn runtime_should_fail_if_non_empty_string_is_returned() {
        let seed = "(This is a message)";
        let result = execute(&mut Client::default(), &format!("function main() return '{seed}' end"));

        assert_eq!(result, Err(ExecutionError::Reported(seed.to_string())));
    }

    #[test]
    fn runtime_should_able_to_call_rusty_client_from_lua() {
        let mut client = Client::default();

        assert!(!client.booster.contains_key("booster_A"));

        let result = execute(
            &mut client,
            r#"
            function main()
                api_boost("booster_A", 0.5);
                return ''
            end
            "#
        );

        assert_eq!(result, Ok(()));
        assert_eq!(client.booster.get("booster_A"), Some(&0.5));
    }
}
