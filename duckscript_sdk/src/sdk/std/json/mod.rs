mod encode;
mod parse;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "json";

pub(crate) static OBJECT_VALUE: &str = "[OBJECT]";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(encode::create(&package))?;
    commands.set(parse::create(&package))?;

    Ok(())
}
