use crate::utils::pckg;
use crate::utils::state::{get_as_string, get_handles_sub_state};
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::env::Env;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;
use std::env;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "SetVar")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["set_env".to_string()]
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
            CommandResult::Error("Missing environment variable name and value.".to_string())
        } else if arguments.len() == 1 {
            CommandResult::Error("Missing environment variable value.".to_string())
        } else if arguments[0].is_empty() {
            CommandResult::Error("Environment variable name is empty string.".to_string())
        } else {
            if arguments[0] == "--handle" {
                let state = get_handles_sub_state(state);

                let key = &arguments[1];

                match state.get(key) {
                    Some(state_value) => match state_value {
                        StateValue::SubState(map) => {
                            for (env_key, env_value) in map {
                                if !env_key.is_empty() {
                                    if let Ok(env_value_string) = get_as_string(env_value) {
                                        env::set_var(&env_key, &env_value_string);
                                    }
                                }
                            }

                            CommandResult::Continue(Some("true".to_string()))
                        }
                        _ => CommandResult::Error("Invalid handle provided.".to_string()),
                    },
                    None => CommandResult::Error(
                        format!("Map for handle: {} not found.", key).to_string(),
                    ),
                }
            } else {
                env::set_var(&arguments[0], &arguments[1]);

                CommandResult::Continue(Some("true".to_string()))
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
