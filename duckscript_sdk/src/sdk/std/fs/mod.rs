mod cat;
mod readfile;

use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "fs";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let mut package = String::from(parent);
    package.push_str("::");
    package.push_str(PACKAGE);

    commands.set(cat::create(&package))?;
    commands.set(readfile::create(&package))?;

    Ok(())
}
