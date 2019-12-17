//! # runner
//!
//! The main entry point which enables running scripts.
//!

#[cfg(test)]
#[path = "./runner_test.rs"]
mod runner_test;

use crate::context::Context;
use crate::error::ScriptError;
use crate::parser;

pub fn run_script_with_context(text: &str, callback: &dyn Fn(Result<(), ScriptError>)) {
    match parser::parse_file(text) {
        Ok(instructions) => {
            let mut context = Context::new();
            context.instructions = Some(instructions);

            callback(Ok(()));
        }
        Err(error) => callback(Err(error)),
    };
}
