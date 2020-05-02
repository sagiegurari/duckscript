mod get_by_name;
pub(crate) mod set;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "var";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(get_by_name::create(&package))?;
    commands.set(set::create(&package))?;

    Ok(())
}
