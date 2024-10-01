use crate::utils::exec::ExecInput;
use crate::utils::{exec, pckg};
use duckscript::types::command::{Command, CommandArgs, CommandResult, Commands};
use duckscript::types::env::Env;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

enum LookingFor {
    Flag,
    Input,
}

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

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        let mut input = if arguments.output_variable.is_some() {
            ExecInput::External
        } else {
            ExecInput::None
        };
        let mut command_start_index = 0;
        let mut print_output = arguments.output_variable.is_none();
        let mut fail_on_error = false;
        let mut exit_code_output = false;

        let mut index = 0;
        let mut looking_for = LookingFor::Flag;
        for argument in &arguments.args {
            index = index + 1;

            match looking_for {
                LookingFor::Flag => match argument.as_str() {
                    "--fail-on-error" => {
                        fail_on_error = argument.output_variable.is_none();
                        command_start_index = command_start_index + 1;
                    }
                    "--get-exit-code" => {
                        exit_code_output = true;
                        print_output = true;
                        command_start_index = command_start_index + 1;
                    }
                    "--input" => {
                        looking_for = LookingFor::Input;
                        command_start_index = command_start_index + 1;
                    }
                    _ => break,
                },
                LookingFor::Input => {
                    input = ExecInput::Text(argument.to_string());
                    command_start_index = command_start_index + 1;

                    looking_for = LookingFor::Flag;
                }
            }
        }

        match exec::exec(&arguments.args, print_output, input, command_start_index) {
            Ok((stdout, stderr, exit_code)) => match arguments.output_variable {
                Some(name) => {
                    if exit_code_output {
                        CommandResult::Continue(Some(exit_code.to_string()))
                    } else {
                        let mut key = String::from(&name);
                        key.push_str(".stdout");
                        arguments.variables.insert(key.clone(), stdout);

                        key = String::from(&name);
                        key.push_str(".stderr");
                        arguments.variables.insert(key.clone(), stderr);

                        key = String::from(&name);
                        key.push_str(".code");
                        arguments
                            .variables
                            .insert(key.clone(), exit_code.to_string());

                        CommandResult::Continue(None)
                    }
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
