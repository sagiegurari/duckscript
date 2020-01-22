pub(crate) mod alias;
pub(crate) mod collections;
mod debug;
mod echo;
mod env;
mod eval;
mod flowcontrol;
mod fs;
mod goto;
mod is_defined;
mod man;
mod math;
mod net;
mod not;
pub(crate) mod on_error;
mod os_family;
mod process;
mod read;
mod release;
mod scope;
mod set;
pub(crate) mod string;
mod test;
mod thread;
mod time;
mod unalias;

use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "std";

pub(crate) fn load(commands: &mut Commands) -> Result<(), ScriptError> {
    commands.set(alias::create(PACKAGE))?;
    commands.set(echo::create(PACKAGE))?;
    commands.set(eval::create(PACKAGE))?;
    commands.set(goto::create(PACKAGE))?;
    commands.set(is_defined::create(PACKAGE))?;
    commands.set(man::create(PACKAGE))?;
    commands.set(not::create(PACKAGE))?;
    commands.set(os_family::create(PACKAGE))?;
    commands.set(read::create(PACKAGE))?;
    commands.set(release::create(PACKAGE))?;
    commands.set(set::create(PACKAGE))?;
    commands.set(unalias::create(PACKAGE))?;

    collections::load(commands, PACKAGE)?;
    debug::load(commands, PACKAGE)?;
    env::load(commands, PACKAGE)?;
    flowcontrol::load(commands, PACKAGE)?;
    fs::load(commands, PACKAGE)?;
    math::load(commands, PACKAGE)?;
    net::load(commands, PACKAGE)?;
    on_error::load(commands, PACKAGE)?;
    process::load(commands, PACKAGE)?;
    scope::load(commands, PACKAGE)?;
    string::load(commands, PACKAGE)?;
    test::load(commands, PACKAGE)?;
    thread::load(commands, PACKAGE)?;
    time::load(commands, PACKAGE)?;

    Ok(())
}
