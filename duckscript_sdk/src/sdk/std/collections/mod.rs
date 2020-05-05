pub(crate) mod array;
mod array_clear;
mod array_concat;
mod array_contains;
mod array_get;
mod array_is_empty;
mod array_join;
pub(crate) mod array_length;
pub(crate) mod array_pop;
mod array_push;
mod array_set;
mod is_array;
mod is_map;
mod is_set;
mod map;
mod map_clear;
mod map_contains_key;
mod map_contains_value;
mod map_get;
mod map_is_empty;
mod map_keys;
mod map_load_properties;
mod map_put;
mod map_remove;
mod map_size;
mod map_to_properties;
mod range;
mod read_properties;
mod set;
mod write_properties;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "collections";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(array::create(&package))?;
    commands.set(array_clear::create(&package))?;
    commands.set(array_concat::create(&package)?)?;
    commands.set(array_contains::create(&package)?)?;
    commands.set(array_get::create(&package))?;
    commands.set(array_push::create(&package))?;
    commands.set(array_set::create(&package))?;
    commands.set(array_is_empty::create(&package)?)?;
    commands.set(array_join::create(&package)?)?;
    commands.set(array_length::create(&package))?;
    commands.set(array_pop::create(&package))?;
    commands.set(is_array::create(&package))?;
    commands.set(is_map::create(&package))?;
    commands.set(is_set::create(&package))?;
    commands.set(map::create(&package))?;
    commands.set(map_clear::create(&package))?;
    commands.set(map_contains_key::create(&package)?)?;
    commands.set(map_contains_value::create(&package)?)?;
    commands.set(map_get::create(&package))?;
    commands.set(map_is_empty::create(&package)?)?;
    commands.set(map_keys::create(&package))?;
    commands.set(map_load_properties::create(&package))?;
    commands.set(map_put::create(&package))?;
    commands.set(map_remove::create(&package))?;
    commands.set(map_size::create(&package))?;
    commands.set(map_to_properties::create(&package))?;
    commands.set(range::create(&package))?;
    commands.set(read_properties::create(&package))?;
    commands.set(set::create(&package))?;
    commands.set(write_properties::create(&package))?;

    Ok(())
}
