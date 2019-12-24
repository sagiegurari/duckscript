mod internal;
mod std;

use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

pub(crate) fn load(commands: &mut Commands) -> Result<(), ScriptError> {
    internal::load(commands)?;
    std::load(commands)?;

    Ok(())
}
