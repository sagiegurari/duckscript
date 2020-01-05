pub(crate) mod alias;
mod collections;
mod echo;
mod env;
mod eval;
mod forin;
mod fs;
mod function;
mod goto;
mod ifelse;
mod is_defined;
mod man;
mod math;
mod net;
mod not;
mod process;
mod read;
mod release;
mod set;
mod string;
mod test;
mod thread;
mod unalias;

use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "sdk";

pub(crate) fn load(commands: &mut Commands) -> Result<(), ScriptError> {
    commands.set(alias::create(PACKAGE))?;
    commands.set(echo::create(PACKAGE))?;
    commands.set(eval::create(PACKAGE))?;
    commands.set(goto::create(PACKAGE))?;
    commands.set(is_defined::create(PACKAGE))?;
    commands.set(man::create(PACKAGE))?;
    commands.set(not::create(PACKAGE))?;
    commands.set(read::create(PACKAGE))?;
    commands.set(release::create(PACKAGE))?;
    commands.set(set::create(PACKAGE))?;
    commands.set(unalias::create(PACKAGE))?;

    collections::load(commands, PACKAGE)?;
    env::load(commands, PACKAGE)?;
    forin::load(commands, PACKAGE)?;
    fs::load(commands, PACKAGE)?;
    function::load(commands, PACKAGE)?;
    ifelse::load(commands, PACKAGE)?;
    math::load(commands, PACKAGE)?;
    net::load(commands, PACKAGE)?;
    process::load(commands, PACKAGE)?;
    string::load(commands, PACKAGE)?;
    test::load(commands, PACKAGE)?;
    thread::load(commands, PACKAGE)?;

    Ok(())
}
