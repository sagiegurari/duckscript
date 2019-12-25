use crate::utils::pckg;
use crate::utils::state::get_sub_state;
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::error::ScriptError;
use duckscript::types::instruction::InstructionMetaInfo;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

struct FunctionCommand {
    package: String,
}

impl Command for FunctionCommand {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Function")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["function".to_string(), "fn".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn requires_context(&self) -> bool {
        true
    }

    fn run_with_context(
        &self,
        state: &mut HashMap<String, StateValue>,
        commands: &mut Commands,
        arguments: Vec<String>,
        meta_info: InstructionMetaInfo,
    ) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("Missing function name.".to_string(), meta_info)
        } else {
            let function_name = arguments[0].clone();
            let fn_state_key = self.name();
            let fn_state = get_sub_state(fn_state_key, state);

            if fn_state.contains_key(&function_name) {
                CommandResult::Error(
                    format!("Function: {} was already defined.", &function_name).to_string(),
                    meta_info,
                )
            } else {
                pub(crate) struct CallFunctionCommand {
                    name: String,
                }

                impl Command for CallFunctionCommand {
                    fn name(&self) -> String {
                        self.name.clone()
                    }

                    fn help(&self) -> String {
                        "".to_string()
                    }

                    fn requires_context(&self) -> bool {
                        true
                    }

                    fn run_with_context(
                        &self,
                        _state: &mut HashMap<String, StateValue>,
                        _commands: &mut Commands,
                        _arguments: Vec<String>,
                        _meta_info: InstructionMetaInfo,
                    ) -> CommandResult {
                        //todo impl this
                        CommandResult::Continue(None)
                    }
                }

                match commands.set(Box::new(CallFunctionCommand {
                    name: function_name.clone(),
                })) {
                    Ok(_) => CommandResult::Continue(None),
                    Err(error) => CommandResult::Error(error.to_string(), meta_info),
                }
            }
        }
    }
}

struct EndFunctionCommand {
    package: String,
}

impl Command for EndFunctionCommand {
    fn name(&self) -> String {
        pckg::concat(&self.package, "EndFunction")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["end_function".to_string(), "end_fn".to_string()]
    }

    fn help(&self) -> String {
        "".to_string()
    }

    fn requires_context(&self) -> bool {
        true
    }

    fn run_with_context(
        &self,
        _state: &mut HashMap<String, StateValue>,
        _commands: &mut Commands,
        _arguments: Vec<String>,
        _meta_info: InstructionMetaInfo,
    ) -> CommandResult {
        CommandResult::Continue(None)
    }
}

pub(crate) fn create(package: &str) -> Vec<Box<dyn Command>> {
    vec![
        Box::new(FunctionCommand {
            package: package.to_string(),
        }),
        Box::new(EndFunctionCommand {
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
