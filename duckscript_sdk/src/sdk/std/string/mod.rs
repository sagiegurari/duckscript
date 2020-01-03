mod contains;
mod ends_with;
mod equals;
mod starts_with;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "string";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(contains::create(&package))?;
    commands.set(ends_with::create(&package))?;
    commands.set(equals::create(&package))?;
    commands.set(starts_with::create(&package))?;

    Ok(())
}
