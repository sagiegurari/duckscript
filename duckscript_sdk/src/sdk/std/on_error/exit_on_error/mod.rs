use crate::sdk::std::on_error::{get_value, EXIT_ON_ERROR_KEY, STATE_KEY};
use crate::utils::state::get_core_sub_state_for_command;
use crate::utils::{condition, pckg};
use duckscript::types::command::{Command, CommandArgs, CommandResult, Commands};
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

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        let exit_on_error = if arguments.args.is_empty() {
            let value_string = get_value(state, EXIT_ON_ERROR_KEY.to_string());
            condition::is_true(value_string)
        } else {
            let exit_on_error = condition::is_true(Some(arguments.args[0].clone()));

            let sub_state = get_core_sub_state_for_command(arguments.state, STATE_KEY.to_string());
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
