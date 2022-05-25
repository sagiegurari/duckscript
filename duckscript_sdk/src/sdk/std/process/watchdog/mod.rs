use crate::utils::exec::ExecInput;
use crate::utils::{exec, pckg};
use duckscript::types::command::{Command, CommandResult};
use std::thread;
use std::time::Duration;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

enum LookingFor {
    Flag,
    MaxRetries,
    Interval,
    Input,
}

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Watchdog")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["watchdog".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("Command not provided.".to_string())
        } else {
            let mut max_retries: isize = -1;
            let mut interval: u64 = 0;
            let mut input = ExecInput::None;
            let mut command_start_index = 0;

            let mut index = 0;
            let mut looking_for = LookingFor::Flag;
            for argument in &arguments {
                index = index + 1;

                match looking_for {
                    LookingFor::Flag => match argument.as_str() {
                        "--" => {
                            command_start_index = index;
                            break;
                        }
                        "--max-retries" => looking_for = LookingFor::MaxRetries,
                        "--interval" => looking_for = LookingFor::Interval,
                        "--input" => looking_for = LookingFor::Input,
                        _ => {
                            return CommandResult::Error(
                                format!("Unexpected argument: {} found", argument).to_string(),
                            );
                        }
                    },
                    LookingFor::MaxRetries => {
                        max_retries = match argument.parse() {
                            Ok(value) => value,
                            Err(_) => {
                                return CommandResult::Error(
                                    format!(
                                        "Max retries value must be positive number, found: {}",
                                        argument
                                    )
                                    .to_string(),
                                );
                            }
                        };

                        looking_for = LookingFor::Flag;
                    }
                    LookingFor::Interval => {
                        interval = match argument.parse() {
                            Ok(value) => value,
                            Err(_) => {
                                return CommandResult::Error(
                                    format!(
                                        "Interval value must be positive number, found: {}",
                                        argument
                                    )
                                    .to_string(),
                                );
                            }
                        };

                        looking_for = LookingFor::Flag;
                    }
                    LookingFor::Input => {
                        input = ExecInput::Text(argument.to_string());
                        looking_for = LookingFor::Flag;
                    }
                }
            }

            if command_start_index == 0 {
                CommandResult::Error("Command not provided.".to_string())
            } else {
                let millis = Duration::from_millis(interval);

                let mut attempt = 0;
                loop {
                    attempt = attempt + 1;

                    match exec::exec(&arguments, false, input.clone(), command_start_index) {
                        Ok(_) => (),
                        Err(error) => return CommandResult::Error(error),
                    }

                    if max_retries <= 0 || attempt > max_retries {
                        break;
                    } else if interval > 0 {
                        thread::sleep(millis);
                    }
                }

                CommandResult::Continue(Some(attempt.to_string()))
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
