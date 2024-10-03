#![deny(
    future_incompatible,
    keyword_idents,
    let_underscore,
    nonstandard_style,
    unused
)]
#![warn(unknown_lints)]

//! # duckscript_cli
//!
//! The duckscript command line executable.
//!
//! This executable enables to run the duckscript runner with the default sdk.
//!
//! # Installation
//! See [main duckscript documentation](https://github.com/sagiegurari/duckscript)
//!
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/duckscript/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/duckscript/blob/master/LICENSE) open source license.
//!

mod linter;

use duckscript::runner;
use duckscript::types::error::ScriptError;
use duckscript::types::runtime::Context;
use std::env;
use std::process::exit;

static VERSION: &str = env!("CARGO_PKG_VERSION");
static AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
static DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    match run_cli() {
        Err(error) => {
            println!("Error: {}", error);
            exit(1);
        }
        _ => (),
    };
}

fn run_cli() -> Result<(), ScriptError> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        run_repl()
    } else if args[1] == "--version" {
        println!(
            "Duckscript Runtime: {}\nDuckscript SDK: {}\nDuckscript CLI: {}",
            duckscript::version(),
            duckscriptsdk::version(),
            VERSION
        );

        Ok(())
    } else if args[1] == "--help" || args[1] == "-h" {
        let usage = include_str!("help.txt");
        println!(
            "duckscript {}\n{}\n{}\n\n{}",
            VERSION, AUTHOR, DESCRIPTION, usage
        );

        Ok(())
    } else {
        let (value, is_file, run) = if args.len() == 2 {
            (args[1].clone(), true, true)
        } else {
            if args[1] == "-e" || args[1] == "--eval" {
                (args[2].clone(), false, true)
            } else if args[1] == "-l" || args[1] == "--lint" {
                (args[2].clone(), true, false)
            } else {
                (args[1].clone(), true, true)
            }
        };

        if run {
            run_script(&value, is_file)
        } else {
            linter::lint_file(&value)
        }
    }
}

fn create_context() -> Result<Context, ScriptError> {
    let mut context = Context::new();
    duckscriptsdk::load(&mut context.commands)?;

    Ok(context)
}

fn run_script(value: &str, is_file: bool) -> Result<(), ScriptError> {
    let context = create_context()?;

    if is_file {
        runner::run_script_file(value, context, None)?;
    } else {
        runner::run_script(value, context, None)?;
    }

    Ok(())
}

fn run_repl() -> Result<(), ScriptError> {
    let context = create_context()?;

    runner::repl(context)?;

    Ok(())
}
