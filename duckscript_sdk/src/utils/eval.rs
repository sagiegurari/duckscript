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
) -> Result<CommandResult, String> {
    if arguments.is_empty() {
        Ok(CommandResult::Continue(None))
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

                Ok(command_result)
            }
            Err(error) => Err(error.to_string()),
        }
    }
}

pub(crate) fn eval_with_error(
    arguments: &Vec<String>,
    state: &mut HashMap<String, StateValue>,
    variables: &mut HashMap<String, String>,
    commands: &mut Commands,
) -> CommandResult {
    match eval(arguments, state, variables, commands) {
        Ok(command_result) => match command_result.clone() {
            CommandResult::Crash(error) => CommandResult::Error(error),
            _ => command_result,
        },
        Err(error) => CommandResult::Error(error.to_string()),
    }
}
