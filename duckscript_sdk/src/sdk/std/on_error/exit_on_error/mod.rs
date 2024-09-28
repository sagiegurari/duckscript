use crate::sdk::std::on_error::{get_value, EXIT_ON_ERROR_KEY, STATE_KEY};
use crate::utils::state::get_core_sub_state_for_command;
use crate::utils::{condition, pckg};
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
        pckg::concat(&self.package, "SetExitOnError")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["exit_on_error".to_string(), "set_exit_on_error".to_string()]
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
        let exit_on_error = if arguments.is_empty() {
            let value_string = get_value(state, EXIT_ON_ERROR_KEY.to_string());
            condition::is_true(value_string)
        } else {
            let exit_on_error = condition::is_true(Some(arguments[0].clone()));

            let sub_state = get_core_sub_state_for_command(state, STATE_KEY.to_string());
            sub_state.insert(
                EXIT_ON_ERROR_KEY.to_string(),
                StateValue::Boolean(exit_on_error),
            );

            exit_on_error
        };

        CommandResult::Continue(Some(exit_on_error.to_string()))
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
