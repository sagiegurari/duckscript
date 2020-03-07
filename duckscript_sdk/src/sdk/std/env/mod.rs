mod get;
mod get_home_dir;
mod get_user_name;
mod is_windows;
mod os_family;
mod print_current_directory;
mod set;
mod set_current_directory;
mod unset;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "env";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(get::create(&package))?;
    commands.set(get_home_dir::create(&package))?;
    commands.set(get_user_name::create(PACKAGE))?;
    commands.set(is_windows::create(PACKAGE)?)?;
    commands.set(os_family::create(PACKAGE))?;
    commands.set(print_current_directory::create(&package))?;
    commands.set(set::create(&package))?;
    commands.set(set_current_directory::create(&package))?;
    commands.set(unset::create(&package))?;

    Ok(())
}
