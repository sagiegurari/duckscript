use duckscript::parser;
use duckscript::runner;
use duckscript::types::command::{CommandResult, Commands};
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./eval_test.rs"]
mod eval_test;

pub(crate) fn eval(
    arguments: &Vec<String>,
    state: &mut HashMap<String, StateValue>,
    variables: &mut HashMap<String, String>,
    commands: &mut Commands,
) -> CommandResult {
    if arguments.is_empty() {
        CommandResult::Continue(None)
    } else {
        let mut line_buffer = String::new();
        for argument in arguments {
            if argument.contains(" ") {
                line_buffer.push('"');
            }
            line_buffer.push_str(argument);
            if argument.contains(" ") {
                line_buffer.push('"');
            }
            line_buffer.push(' ');
        }

        let line_str = line_buffer.replace("\r", "").replace("\n", "");

        match parser::parse_text(&line_str) {
            Ok(instructions) => {
                let (command_result, _) = runner::run_instruction(
                    commands,
                    variables,
                    state,
                    &vec![],
                    instructions[0].clone(),
                    0,
                );

                command_result
            }
            Err(error) => CommandResult::Error(error.to_string()),
        }
    }
}
