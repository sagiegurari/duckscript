mod hostname;
mod http_client;
mod wget;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "net";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(hostname::create(&package))?;
    commands.set(http_client::create(&package))?;
    commands.set(wget::create(&package)?)?;

    Ok(())
}
