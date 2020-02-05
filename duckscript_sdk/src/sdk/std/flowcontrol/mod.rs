mod end;
mod forin;
mod function;
mod goto;
mod ifelse;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "flowcontrol";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(end::create())?;
    commands.set(goto::create(&package))?;

    forin::load(commands, &package)?;
    function::load(commands, &package)?;
    ifelse::load(commands, &package)?;

    Ok(())
}
