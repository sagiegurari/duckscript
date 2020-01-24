pub(crate) mod exit_on_error;
mod get_last_error;
mod get_last_error_line;
mod get_last_error_source;
mod on_error;
mod set_error;

use crate::utils::pckg;
use crate::utils::state::get_core_sub_state_for_command;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

static PACKAGE: &str = "error";
pub(crate) static STATE_KEY: &str = "on_error";
pub(crate) static EXIT_ON_ERROR_KEY: &str = "exit_on_error";

pub(crate) fn get_value(state: &mut HashMap<String, StateValue>, key: String) -> Option<String> {
    let sub_state = get_core_sub_state_for_command(state, STATE_KEY.to_string());

    match sub_state.get(&key) {
        Some(state_value) => match state_value {
            StateValue::String(value) => Some(value.clone()),
            StateValue::Boolean(value) => Some(value.to_string()),
            _ => None,
        },
        None => None,
    }
}

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(exit_on_error::create(&package))?;
    commands.set(get_last_error::create(&package))?;
    commands.set(get_last_error_line::create(&package))?;
    commands.set(get_last_error_source::create(&package))?;
    commands.set(on_error::create(&package))?;
    commands.set(set_error::create(&package))?;

    Ok(())
}
