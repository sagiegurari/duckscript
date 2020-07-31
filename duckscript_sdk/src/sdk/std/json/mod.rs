mod encode;
mod parse;

use crate::types::command::create_doc_only_command;
use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "json";

pub(crate) static OBJECT_VALUE: &str = "[OBJECT]";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(create_doc_only_command(&package, include_str!("help.md")))?;

    commands.set(encode::create(&package))?;
    commands.set(parse::create(&package))?;

    Ok(())
}
