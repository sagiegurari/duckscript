use crate::utils::{exec, pckg};
use duckscript::types::command::{Command, CommandResult, Commands};
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
        pckg::concat(&self.package, "Execute")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["exec".to_string()]
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
        _state: &mut HashMap<String, StateValue>,
        variables: &mut HashMap<String, String>,
        output_variable: Option<String>,
        _instructions: &Vec<Instruction>,
        _commands: &mut Commands,
        _line: usize,
    ) -> CommandResult {
        let allow_input = output_variable.is_some();
        let print_output = !allow_input;
        let (start_index, fail_on_error) =
            if !arguments.is_empty() && arguments[0] == "--fail-on-error" {
                if output_variable.is_some() {
                    (1, true)
                } else {
                    (1, false)
                }
            } else {
                (0, false)
            };

        match exec::exec(&arguments, print_output, allow_input, start_index) {
            Ok((stdout, stderr, exit_code)) => match output_variable {
                Some(name) => {
                    let mut key = String::from(&name);
                    key.push_str(".stdout");
                    variables.insert(key.clone(), stdout);

                    key = String::from(&name);
                    key.push_str(".stderr");
                    variables.insert(key.clone(), stderr);

                    key = String::from(&name);
                    key.push_str(".code");
                    variables.insert(key.clone(), exit_code.to_string());

                    CommandResult::Continue(None)
                }
                None => {
                    if fail_on_error && exit_code != 0 {
                        CommandResult::Error(
                            format!("Error while executing command, exit code: {}", exit_code)
                                .to_string(),
                        )
                    } else {
                        CommandResult::Continue(None)
                    }
                }
            },
            Err(error) => CommandResult::Error(error),
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
