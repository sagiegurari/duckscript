mod calc;
mod greater_than;
mod hex_decode;
mod hex_encode;
mod less_than;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "math";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(calc::create(&package))?;
    commands.set(greater_than::create(&package))?;
    commands.set(hex_encode::create(&package))?;
    commands.set(hex_decode::create(&package))?;
    commands.set(less_than::create(&package))?;

    Ok(())
}
