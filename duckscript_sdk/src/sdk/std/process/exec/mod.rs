use crate::utils::pckg;
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;
use std::process::Command as ProcessCommand;
use std::process::Stdio;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

struct CommandImpl {
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
        if arguments.is_empty() {
            CommandResult::Error("Command provided.".to_string())
        } else {
            let mut command = ProcessCommand::new(&arguments[0]);

            let mut skip = true;
            for argument in &arguments {
                if skip {
                    skip = false;
                } else {
                    command.arg(argument);
                }
            }

            command.stdin(Stdio::inherit());
            if output_variable.is_none() {
                command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
            }

            match command.output() {
                Ok(ref output) => {
                    match output_variable {
                        Some(name) => {
                            let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
                            let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
                            let exit_code = match output.status.code() {
                                Some(value) => value,
                                None => {
                                    return CommandResult::Error(
                                        format!(
                                            "Unable to extract exit code for command: {}",
                                            &arguments[0]
                                        )
                                        .to_string(),
                                    );
                                }
                            };

                            let mut key = String::from(&name);
                            key.push_str(".stdout");
                            variables.insert(key.clone(), stdout);

                            key = String::from(&name);
                            key.push_str(".stderr");
                            variables.insert(key.clone(), stderr);

                            key = String::from(&name);
                            key.push_str(".code");
                            variables.insert(key.clone(), exit_code.to_string());
                        }
                        None => (),
                    };

                    CommandResult::Continue(None)
                }
                Err(_) => CommandResult::Error(
                    format!("Unable to run command: {}", &arguments[0]).to_string(),
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
