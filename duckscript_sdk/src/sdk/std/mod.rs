pub(crate) mod alias;
mod array;
mod echo;
mod env;
mod eval;
mod forin;
mod fs;
mod function;
mod goto;
mod ifelse;
mod math;
mod not;
mod process;
mod release;
mod set;
mod test;
mod thread;
mod unalias;

use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "sdk";

pub(crate) fn load(commands: &mut Commands) -> Result<(), ScriptError> {
    commands.set(alias::create(PACKAGE))?;
    commands.set(array::create(PACKAGE))?;
    commands.set(echo::create(PACKAGE))?;
    commands.set(eval::create(PACKAGE))?;
    commands.set(goto::create(PACKAGE))?;
    commands.set(not::create(PACKAGE))?;
    commands.set(release::create(PACKAGE))?;
    commands.set(set::create(PACKAGE))?;
    commands.set(unalias::create(PACKAGE))?;

    env::load(commands, PACKAGE)?;
    forin::load(commands, PACKAGE)?;
    fs::load(commands, PACKAGE)?;
    function::load(commands, PACKAGE)?;
    ifelse::load(commands, PACKAGE)?;
    math::load(commands, PACKAGE)?;
    process::load(commands, PACKAGE)?;
    test::load(commands, PACKAGE)?;
    thread::load(commands, PACKAGE)?;

    Ok(())
}
