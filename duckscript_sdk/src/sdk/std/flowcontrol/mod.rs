mod end;
mod forin;
mod function;
mod ifelse;

use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

pub(crate) fn load(commands: &mut Commands, package: &str) -> Result<(), ScriptError> {
    commands.set(end::create())?;

    forin::load(commands, package)?;
    function::load(commands, package)?;
    ifelse::load(commands, package)?;

    Ok(())
}
