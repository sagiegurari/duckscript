mod set;
mod unset;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

pub(crate) static ALIAS_STATE_KEY: &str = "ALIAS_STATE";
static PACKAGE: &str = "alias";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(set::create(&package))?;
    commands.set(unset::create(&package))?;

    Ok(())
}
