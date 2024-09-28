use crate::sdk::std::on_error::{get_value, EXIT_ON_ERROR_KEY, STATE_KEY};
use crate::utils::state::get_core_sub_state_for_command;
use crate::utils::{condition, pckg};
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::env::Env;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "OnError")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["on_error".to_string()]
    }

    fn help(&self) -> String {
        "".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn requires_context(&self) -> bool {
        true
    }

    fn run_with_context(
        &self,
        arguments: Vec<String>,
        state: &mut HashMap<String, StateValue>,
        _variables: &mut HashMap<String, String>,
        _output_variable: Option<String>,
        _instructions: &Vec<Instruction>,
        _commands: &mut Commands,
        _line: usize,
        _env: &mut Env,
    ) -> CommandResult {
        if !arguments.is_empty() {
            let error = arguments[0].clone();

            let exit_on_error = get_value(state, EXIT_ON_ERROR_KEY.to_string());
            let should_crash = condition::is_true(exit_on_error);

            if should_crash {
                CommandResult::Crash(error)
            } else {
                let (line, source) = if arguments.len() > 1 {
                    let line = arguments[1].clone();
                    let source = if arguments.len() > 2 {
                        arguments[2].clone()
                    } else {
                        "".to_string()
                    };

                    (line, source)
                } else {
                    ("".to_string(), "".to_string())
                };

                let sub_state = get_core_sub_state_for_command(state, STATE_KEY.to_string());
                sub_state.insert("error".to_string(), StateValue::String(error));
                sub_state.insert("line".to_string(), StateValue::String(line));
                sub_state.insert("source".to_string(), StateValue::String(source));

                CommandResult::Continue(Some("false".to_string()))
            }
        } else {
            CommandResult::Crash("Invalid input provided.".to_string())
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
