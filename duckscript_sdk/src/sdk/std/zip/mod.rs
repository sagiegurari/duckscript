mod pack;

use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;
use crate::utils::pckg;

static PACKAGE: &str = "zip";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(pack::create(&package))?;

    Ok(())
}
