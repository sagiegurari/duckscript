mod append;
mod basename;
mod canonical;
mod cp;
mod cp_glob;
mod dirname;
mod exists;
mod get_last_modified_time;
mod get_file_size;
mod gitignore_path_array;
mod glob_array;
mod is_directory;
mod is_file;
mod is_path_newer;
mod is_readonly;
mod join_path;
mod list;
mod mkdir;
mod mv;
mod print;
mod read_bytes;
pub(crate) mod read_text;
mod rm;
mod rmdir;
mod set_mode;
mod set_mode_glob;
mod temp_dir;
mod temp_file;
mod touch;
mod write_bytes;
mod write_text;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "fs";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(append::create(&package))?;
    commands.set(basename::create(&package))?;
    commands.set(canonical::create(&package))?;
    commands.set(cp::create(&package))?;
    commands.set(cp_glob::create(&package)?)?;
    commands.set(dirname::create(&package))?;
    commands.set(exists::create(&package))?;
    commands.set(get_last_modified_time::create(&package))?;
    commands.set(get_file_size::create(&package))?;
    commands.set(gitignore_path_array::create(&package))?;
    commands.set(glob_array::create(&package))?;
    commands.set(is_directory::create(&package))?;
    commands.set(is_file::create(&package))?;
    commands.set(is_path_newer::create(&package))?;
    commands.set(is_readonly::create(&package))?;
    commands.set(join_path::create(&package)?)?;
    commands.set(list::create(&package))?;
    commands.set(mkdir::create(&package))?;
    commands.set(mv::create(&package))?;
    commands.set(print::create(&package))?;
    commands.set(read_bytes::create(&package))?;
    commands.set(read_text::create(&package))?;
    commands.set(rm::create(&package))?;
    commands.set(rmdir::create(&package))?;
    commands.set(set_mode::create(&package))?;
    commands.set(set_mode_glob::create(&package)?)?;
    commands.set(temp_dir::create(&package))?;
    commands.set(temp_file::create(&package))?;
    commands.set(touch::create(&package))?;
    commands.set(write_bytes::create(&package))?;
    commands.set(write_text::create(&package))?;

    Ok(())
}
