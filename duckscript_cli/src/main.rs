use duckscript::runner;
use duckscript::types::error::ScriptError;
use duckscript::types::runtime::Context;
use duckscriptsdk;
use std::env;

fn main() {
    match run() {
        Err(error) => panic!("{}", error),
        _ => (),
    };
}

fn run() -> Result<(), ScriptError> {
    println!("test cli...."); //todo remove
    match env::current_dir() {
        Ok(path) => println!("The current directory is {}", path.display()),
        _ => (),
    };

    let mut context = Context::new();
    duckscriptsdk::load(&mut context.commands)?;

    runner::run_script_file("./duckscript/src/test/scripts/simple_runnable.ds", context)?;

    Ok(())
}
