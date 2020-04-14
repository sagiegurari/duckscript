mod env_to_map;
mod get_env;
mod get_home_dir;
mod get_user_name;
mod is_windows;
mod os_family;
mod os_name;
mod os_release;
mod os_version;
mod print_current_directory;
mod print_env;
mod set_current_directory;
mod set_env;
mod uname;
mod unset;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "env";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(env_to_map::create(&package))?;
    commands.set(get_env::create(&package))?;
    commands.set(get_home_dir::create(&package))?;
    commands.set(get_user_name::create(&package))?;
    commands.set(is_windows::create(&package)?)?;
    commands.set(os_family::create(&package))?;
    commands.set(os_name::create(&package))?;
    commands.set(os_release::create(&package))?;
    commands.set(os_version::create(&package))?;
    commands.set(print_current_directory::create(&package))?;
    commands.set(print_env::create(&package)?)?;
    commands.set(set_current_directory::create(&package))?;
    commands.set(set_env::create(&package))?;
    commands.set(uname::create(&package)?)?;
    commands.set(unset::create(&package))?;

    Ok(())
}
