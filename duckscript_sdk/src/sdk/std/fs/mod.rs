mod print;
mod read;
mod write;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "fs";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(print::create(&package))?;
    commands.set(read::create(&package))?;
    commands.set(write::create(&package))?;

    Ok(())
}
