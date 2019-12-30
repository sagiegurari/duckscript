mod cd;
mod get;
mod pwd;
mod set;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "env";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(cd::create(&package))?;
    commands.set(get::create(&package))?;
    commands.set(pwd::create(&package))?;
    commands.set(set::create(&package))?;

    Ok(())
}
