mod concat;
mod contains;
mod ends_with;
pub(crate) mod equals;
mod indexof;
mod is_empty;
mod last_indexof;
mod length;
mod replace;
mod split;
mod starts_with;
mod substring;
mod trim;
mod trim_end;
mod trim_start;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "string";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(concat::create(&package)?)?;
    commands.set(contains::create(&package))?;
    commands.set(ends_with::create(&package))?;
    commands.set(equals::create(&package))?;
    commands.set(indexof::create(&package))?;
    commands.set(is_empty::create(&package))?;
    commands.set(last_indexof::create(&package))?;
    commands.set(length::create(&package))?;
    commands.set(replace::create(&package))?;
    commands.set(split::create(&package))?;
    commands.set(starts_with::create(&package))?;
    commands.set(substring::create(&package))?;
    commands.set(trim::create(&package))?;
    commands.set(trim_start::create(&package))?;
    commands.set(trim_end::create(&package))?;

    Ok(())
}
