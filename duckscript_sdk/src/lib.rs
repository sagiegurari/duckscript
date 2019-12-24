mod sdk;
mod utils;

use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

pub fn load(commands: &mut Commands) -> Result<(), ScriptError> {
    sdk::load(commands)
}
