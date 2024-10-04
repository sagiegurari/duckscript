use crate::utils::exec::ExecInput;
use crate::utils::{exec, pckg};
use duckscript::types::command::{Command, CommandInvocationContext, CommandResult};

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
        pckg::concat(&self.package, "Spawn")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["spawn".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        let mut print_output = true;
        let mut input = ExecInput::None;
        let mut command_start_index = 0;

        let mut index = 0;
        let mut looking_for = LookingFor::Flag;
        for argument in &context.arguments {
            index = index + 1;

            match looking_for {
                LookingFor::Flag => match argument.as_str() {
                    "--silent" => {
                        print_output = false;
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

        match exec::spawn(
            &context.arguments,
            print_output,
            true,
            input,
            command_start_index,
        ) {
            Ok(child) => {
                let pid = child.id();

                CommandResult::Continue(Some(pid.to_string()))
            }
            Err(error) => CommandResult::Error(error),
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
