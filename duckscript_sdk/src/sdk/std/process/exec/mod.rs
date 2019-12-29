use crate::utils::pckg;
use crate::utils::state::{get_handle, put_handle};
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::error::ScriptError;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;
use std::process::Command as ProcessCommand;
use std::process::Stdio;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

struct ExecuteOutput {
    stdout: String,
    stderr: String,
    exit_code: i32,
}

fn serialize_execute_output(
    output: &ExecuteOutput,
    state: &mut HashMap<String, StateValue>,
) -> String {
    let mut sub_state = HashMap::new();

    sub_state.insert(
        "stdout".to_string(),
        StateValue::String(output.stdout.clone()),
    );
    sub_state.insert(
        "stderr".to_string(),
        StateValue::String(output.stderr.clone()),
    );
    sub_state.insert(
        "exit_code".to_string(),
        StateValue::Number32Bit(output.exit_code),
    );

    put_handle(state, StateValue::SubState(sub_state))
}

fn deserialize_execute_output(
    handle: String,
    state: &mut HashMap<String, StateValue>,
) -> Option<ExecuteOutput> {
    match get_handle(state, handle) {
        Some(handle_value) => match handle_value {
            StateValue::SubState(sub_state) => {
                let stdout = match sub_state.get("stdout") {
                    Some(state_value) => match state_value {
                        StateValue::String(value) => value.to_string(),
                        _ => return None,
                    },
                    None => return None,
                };
                let stderr = match sub_state.get("stderr") {
                    Some(state_value) => match state_value {
                        StateValue::String(value) => value.to_string(),
                        _ => return None,
                    },
                    None => return None,
                };
                let exit_code = match sub_state.get("exit_code") {
                    Some(state_value) => match state_value {
                        StateValue::Number32Bit(value) => *value,
                        _ => return None,
                    },
                    None => return None,
                };

                Some(ExecuteOutput {
                    stdout,
                    stderr,
                    exit_code,
                })
            }
            _ => return None,
        },
        None => None,
    }
}

struct ExecuteCommand {
    package: String,
}

impl Command for ExecuteCommand {
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
        state: &mut HashMap<String, StateValue>,
        _variables: &mut HashMap<String, String>,
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

                    if output_variable.is_none() {
                        CommandResult::Continue(Some(exit_code.to_string()))
                    } else {
                        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
                        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

                        let execute_output = ExecuteOutput {
                            stdout,
                            stderr,
                            exit_code,
                        };

                        let handle = serialize_execute_output(&execute_output, state);

                        CommandResult::Continue(Some(handle))
                    }
                }
                Err(_) => CommandResult::Error(
                    format!("Unable to run command: {}", &arguments[0]).to_string(),
                ),
            }
        }
    }
}

struct GetStdOutputCommand {
    package: String,
}

impl Command for GetStdOutputCommand {
    fn name(&self) -> String {
        pckg::concat(&self.package, "GetStdOutput")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["get_stdout".to_string()]
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
        if arguments.is_empty() {
            CommandResult::Error("Handle not provided.".to_string())
        } else {
            match deserialize_execute_output(arguments[0].clone(), state) {
                Some(execute_output) => {
                    CommandResult::Continue(Some(execute_output.stdout.clone()))
                }
                None => CommandResult::Continue(None),
            }
        }
    }
}

struct GetStdErrorCommand {
    package: String,
}

impl Command for GetStdErrorCommand {
    fn name(&self) -> String {
        pckg::concat(&self.package, "getStdError")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["get_stderr".to_string()]
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
        if arguments.is_empty() {
            CommandResult::Error("Handle not provided.".to_string())
        } else {
            match deserialize_execute_output(arguments[0].clone(), state) {
                Some(execute_output) => {
                    CommandResult::Continue(Some(execute_output.stderr.clone()))
                }
                None => CommandResult::Continue(None),
            }
        }
    }
}

struct GetExitCodeCommand {
    package: String,
}

impl Command for GetExitCodeCommand {
    fn name(&self) -> String {
        pckg::concat(&self.package, "GetExitCode")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["get_exit_code".to_string()]
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
        if arguments.is_empty() {
            CommandResult::Error("Handle not provided.".to_string())
        } else {
            match deserialize_execute_output(arguments[0].clone(), state) {
                Some(execute_output) => {
                    CommandResult::Continue(Some(execute_output.exit_code.to_string()))
                }
                None => CommandResult::Continue(None),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Vec<Box<dyn Command>> {
    vec![
        Box::new(ExecuteCommand {
            package: package.to_string(),
        }),
        Box::new(GetStdOutputCommand {
            package: package.to_string(),
        }),
        Box::new(GetStdErrorCommand {
            package: package.to_string(),
        }),
        Box::new(GetExitCodeCommand {
            package: package.to_string(),
        }),
    ]
}

pub(crate) fn load(commands: &mut Commands, package: &str) -> Result<(), ScriptError> {
    let multi_commands = create(package);

    for command in multi_commands {
        commands.set(command)?;
    }

    Ok(())
}
