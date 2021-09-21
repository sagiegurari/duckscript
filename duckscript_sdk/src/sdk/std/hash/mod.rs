mod digest;
mod sha256sum;
mod sha512sum;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "hash";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(digest::create(&package))?;
    commands.set(sha256sum::create(&package)?)?;
    commands.set(sha512sum::create(&package)?)?;

    Ok(())
}
