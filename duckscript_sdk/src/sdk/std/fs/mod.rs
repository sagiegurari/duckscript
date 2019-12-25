mod read;
mod print;

use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "fs";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let mut package = String::from(parent);
    package.push_str("::");
    package.push_str(PACKAGE);

    commands.set(print::create(&package))?;
    commands.set(read::create(&package))?;

    Ok(())
}
