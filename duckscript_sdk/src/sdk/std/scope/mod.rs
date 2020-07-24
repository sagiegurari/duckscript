pub(crate) mod clear;
mod pop_stack;
mod push_stack;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "scope";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(clear::create(&package))?;
    commands.set(pop_stack::create(&package))?;
    commands.set(push_stack::create(&package))?;

    Ok(())
}
