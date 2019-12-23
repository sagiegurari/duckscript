pub mod echo;
pub mod set;

use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

pub(crate) fn load(commands: &mut Commands) -> Result<(), ScriptError> {
    commands.set(echo::create())?;
    commands.set(set::create())?;

    Ok(())
}
