use crate::sdk::std::on_error;
use crate::utils::pckg;
use crate::utils::state::get_core_sub_state_for_command;
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

struct CommandImpl {
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
    ) -> CommandResult {
        if !arguments.is_empty() {
            let error = arguments[0].clone();
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

            let sub_state = get_core_sub_state_for_command(state, on_error::STATE_KEY.to_string());
            sub_state.insert("error".to_string(), StateValue::String(error));
            sub_state.insert("line".to_string(), StateValue::String(line));
            sub_state.insert("source".to_string(), StateValue::String(source));

            CommandResult::Continue(Some("false".to_string()))
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
