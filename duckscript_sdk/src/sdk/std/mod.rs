mod echo;
mod env;
mod eval;
mod fs;
mod function;
mod goto;
mod ifelse;
mod set;

use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "sdk";

pub(crate) fn load(commands: &mut Commands) -> Result<(), ScriptError> {
    commands.set(echo::create(PACKAGE))?;
    commands.set(eval::create(PACKAGE))?;
    commands.set(goto::create(PACKAGE))?;
    commands.set(set::create(PACKAGE))?;

    env::load(commands, PACKAGE)?;
    fs::load(commands, PACKAGE)?;
    function::load(commands, PACKAGE)?;
    ifelse::load(commands, PACKAGE)?;

    Ok(())
}
