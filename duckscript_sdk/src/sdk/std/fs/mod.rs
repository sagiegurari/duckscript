mod append;
mod basename;
mod canonical;
mod cp;
mod dirname;
mod exists;
mod is_directory;
mod is_file;
mod list;
mod mkdir;
mod mv;
mod print;
mod read_bytes;
mod read_text;
mod rm;
mod rmdir;
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
    commands.set(dirname::create(&package))?;
    commands.set(exists::create(&package))?;
    commands.set(is_directory::create(&package))?;
    commands.set(is_file::create(&package))?;
    commands.set(list::create(&package))?;
    commands.set(mkdir::create(&package))?;
    commands.set(mv::create(&package))?;
    commands.set(print::create(&package))?;
    commands.set(read_bytes::create(&package))?;
    commands.set(read_text::create(&package))?;
    commands.set(rm::create(&package))?;
    commands.set(rmdir::create(&package))?;
    commands.set(temp_file::create(&package))?;
    commands.set(touch::create(&package))?;
    commands.set(write_bytes::create(&package))?;
    commands.set(write_text::create(&package))?;

    Ok(())
}
