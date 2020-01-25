mod cd;
mod get;
mod get_home_dir;
mod pwd;
mod set;
mod unset;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "env";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(cd::create(&package))?;
    commands.set(get::create(&package))?;
    commands.set(get_home_dir::create(&package))?;
    commands.set(pwd::create(&package))?;
    commands.set(set::create(&package))?;
    commands.set(unset::create(&package))?;

    Ok(())
}
