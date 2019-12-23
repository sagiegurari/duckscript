pub mod std;

use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

pub fn load(commands: &mut Commands) -> Result<(), ScriptError> {
    std::load(commands)
}
