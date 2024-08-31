use crate::utils::state::get_handles_sub_state;
use crate::utils::{io, pckg};
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
        pckg::concat(&self.package, "WriteBytes")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["writebinfile".to_string(), "write_binary_file".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
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
        if arguments.is_empty() {
            CommandResult::Error("File name and text not provided.".to_string())
        } else if arguments.len() == 1 {
            CommandResult::Error("Binary data handle not provided.".to_string())
        } else {
            let state = get_handles_sub_state(state);

            let key = &arguments[1];

            match state.get(key) {
                Some(state_value) => match state_value {
                    StateValue::ByteArray(binary) => {
                        let result = io::write_to_file(&arguments[0], &binary, false);

                        match result {
                            Ok(_) => CommandResult::Continue(Some("true".to_string())),
                            Err(_) => CommandResult::Continue(Some("false".to_string())),
                        }
                    }
                    _ => CommandResult::Error("Invalid handle provided.".to_string()),
                },
                None => CommandResult::Error(
                    format!("Array for handle: {} not found.", key).to_string(),
                ),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
