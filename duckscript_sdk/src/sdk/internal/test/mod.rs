mod test_directory;
mod test_file;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "test";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(test_directory::create(&package))?;
    commands.set(test_file::create(&package))?;

    Ok(())
}
