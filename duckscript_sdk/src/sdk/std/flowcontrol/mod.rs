mod end;
mod forin;
mod function;
mod goto;
mod ifelse;
mod while_mod;

use crate::types::scope::get_line_context_name;
use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

static PACKAGE: &str = "flowcontrol";

fn get_line_key(line: usize, state: &mut HashMap<String, StateValue>) -> String {
    let mut key = get_line_context_name(state);
    key.push_str("::");
    key.push_str(&line.to_string());

    key
}

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(end::create())?;
    commands.set(goto::create(&package))?;

    forin::load(commands, &package)?;
    function::load(commands, &package)?;
    ifelse::load(commands, &package)?;
    while_mod::load(commands, &package)?;

    Ok(())
}
