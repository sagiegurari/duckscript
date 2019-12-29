use crate::utils::state::get_handles_sub_state;
use duckscript::runner;
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::error::ScriptError;
use duckscript::types::instruction::{Instruction, InstructionMetaInfo};
use duckscript::types::runtime::{Context, StateValue};
use std::collections::HashMap;

pub(crate) struct SetCommand {}

impl Command for SetCommand {
    fn name(&self) -> String {
        "test_set".to_string()
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Continue(None)
        } else {
            CommandResult::Continue(Some(arguments[0].clone()))
        }
    }
}

pub(crate) struct SetHandleCommand {}

impl Command for SetHandleCommand {
    fn name(&self) -> String {
        "test_set_handle".to_string()
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
        _meta_info: InstructionMetaInfo,
        _line: usize,
    ) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Continue(None)
        } else {
            let state = get_handles_sub_state(state);
            state.insert(arguments[0].clone(), StateValue::String("test".to_string()));
            CommandResult::Continue(None)
        }
    }
}

pub(crate) enum CommandValidation {
    None,
    Match(String, String),
    Contains(String, String),
}

pub(crate) fn test_common_command_functions(command: Box<dyn Command>) {
    assert!(command.name().len() > 0);
    command.help();
    command.aliases();
}

fn run_command(commands: Vec<Box<dyn Command>>, script: &str) -> Result<Context, ScriptError> {
    let mut context = Context::new();
    for command in commands {
        context.commands.set(command)?;
    }
    runner::run_script(script, context)
}

pub(crate) fn run_script_and_fail(commands: Vec<Box<dyn Command>>, script: &str) {
    let result = run_command(commands, script);
    assert!(result.is_err());
}

pub(crate) fn run_script_and_validate(
    commands: Vec<Box<dyn Command>>,
    script: &str,
    validation: CommandValidation,
) -> Context {
    let result = run_command(commands, script);
    match result {
        Ok(context) => {
            match validation {
                CommandValidation::None => assert!(context.variables.is_empty()),
                CommandValidation::Match(key, value) => {
                    assert_eq!(context.variables.get(&key), Some(&value))
                }
                CommandValidation::Contains(key, value) => {
                    let var_value = context.variables.get(&key).unwrap();
                    assert!(
                        var_value.contains(&value),
                        "The value: {} is not contained in: {}",
                        &value,
                        &var_value
                    )
                }
            };

            context
        }
        Err(error) => panic!(error.to_string()),
    }
}
