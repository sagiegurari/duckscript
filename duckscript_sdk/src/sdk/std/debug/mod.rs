mod dump_instructions;
mod dump_state;
mod dump_variables;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "debug";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(dump_instructions::create(&package))?;
    commands.set(dump_state::create(&package))?;
    commands.set(dump_variables::create(&package))?;

    Ok(())
}
