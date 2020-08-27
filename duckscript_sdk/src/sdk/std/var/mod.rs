mod get_all_var_names;
mod get_by_name;
mod is_defined;
pub(crate) mod set;
mod set_by_name;
mod unset;
mod unset_all_vars;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "var";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(get_all_var_names::create(&package))?;
    commands.set(get_by_name::create(&package))?;
    commands.set(is_defined::create(PACKAGE))?;
    commands.set(set::create(&package))?;
    commands.set(set_by_name::create(&package))?;
    commands.set(unset::create(&package)?)?;
    commands.set(unset_all_vars::create(&package))?;

    Ok(())
}
