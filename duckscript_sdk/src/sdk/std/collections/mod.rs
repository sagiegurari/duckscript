pub(crate) mod array;
mod array_is_empty;
pub(crate) mod array_length;
mod array_pop;
mod is_array;
mod range;
mod read_properties;
mod write_properties;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "collections";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(array::create(&package))?;
    commands.set(array_is_empty::create(&package)?)?;
    commands.set(array_length::create(&package))?;
    commands.set(array_pop::create(&package))?;
    commands.set(is_array::create(&package))?;
    commands.set(range::create(&package))?;
    commands.set(read_properties::create(&package))?;
    commands.set(write_properties::create(&package))?;

    Ok(())
}
