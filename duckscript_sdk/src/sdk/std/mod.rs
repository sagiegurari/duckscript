pub(crate) mod collections;
mod debug;
mod echo;
mod env;
mod eval;
mod flowcontrol;
mod fs;
mod is_defined;
mod json;
mod lib;
mod man;
mod math;
mod net;
mod noop;
mod not;
pub(crate) mod on_error;
mod process;
mod read;
pub(crate) mod release;
pub(crate) mod scope;
pub(crate) mod string;
mod test;
mod thread;
mod time;
mod var;

use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "std";

pub(crate) fn load(commands: &mut Commands) -> Result<(), ScriptError> {
    commands.set(echo::create(PACKAGE))?;
    commands.set(eval::create(PACKAGE))?;
    commands.set(is_defined::create(PACKAGE))?;
    commands.set(man::create(PACKAGE))?;
    commands.set(noop::create(PACKAGE))?;
    commands.set(not::create(PACKAGE))?;
    commands.set(read::create(PACKAGE))?;
    commands.set(release::create(PACKAGE))?;

    collections::load(commands, PACKAGE)?;
    debug::load(commands, PACKAGE)?;
    env::load(commands, PACKAGE)?;
    flowcontrol::load(commands, PACKAGE)?;
    fs::load(commands, PACKAGE)?;
    json::load(commands, PACKAGE)?;
    lib::load(commands, PACKAGE)?;
    math::load(commands, PACKAGE)?;
    net::load(commands, PACKAGE)?;
    on_error::load(commands, PACKAGE)?;
    process::load(commands, PACKAGE)?;
    scope::load(commands, PACKAGE)?;
    string::load(commands, PACKAGE)?;
    test::load(commands, PACKAGE)?;
    thread::load(commands, PACKAGE)?;
    time::load(commands, PACKAGE)?;
    var::load(commands, PACKAGE)?;

    Ok(())
}
