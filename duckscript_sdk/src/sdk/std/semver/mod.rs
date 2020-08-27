mod is_equal;
mod is_newer;
mod parse;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "semver";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(is_equal::create(&package))?;
    commands.set(is_newer::create(&package))?;
    commands.set(parse::create(&package))?;

    Ok(())
}
