pub(crate) mod array;
mod array_concat;
mod array_is_empty;
mod array_join;
pub(crate) mod array_length;
pub(crate) mod array_pop;
mod array_push;
mod is_array;
mod map;
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
    commands.set(array_concat::create(&package)?)?;
    commands.set(array_push::create(&package))?;
    commands.set(array_is_empty::create(&package)?)?;
    commands.set(array_join::create(&package)?)?;
    commands.set(array_length::create(&package))?;
    commands.set(array_pop::create(&package))?;
    commands.set(is_array::create(&package))?;
    commands.set(map::create(&package))?;
    commands.set(range::create(&package))?;
    commands.set(read_properties::create(&package))?;
    commands.set(write_properties::create(&package))?;

    Ok(())
}
