mod end;
mod forin;
mod function;
mod goto;
mod ifelse;

use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

pub(crate) fn load(commands: &mut Commands, package: &str) -> Result<(), ScriptError> {
    commands.set(end::create())?;
    commands.set(goto::create(package))?;

    forin::load(commands, package)?;
    function::load(commands, package)?;
    ifelse::load(commands, package)?;

    Ok(())
}
