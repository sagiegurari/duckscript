pub(crate) mod array;
mod array_length;
mod range;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "collections";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(array::create(&package))?;
    commands.set(array_length::create(&package))?;
    commands.set(range::create(&package))?;

    Ok(())
}
