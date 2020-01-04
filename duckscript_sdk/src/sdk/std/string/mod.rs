mod contains;
mod ends_with;
mod equals;
mod is_empty;
mod starts_with;
mod trim;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "string";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(contains::create(&package))?;
    commands.set(ends_with::create(&package))?;
    commands.set(equals::create(&package))?;
    commands.set(is_empty::create(&package))?;
    commands.set(starts_with::create(&package))?;
    commands.set(trim::create(&package))?;

    Ok(())
}
