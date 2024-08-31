use crate::sdk::std::on_error::STATE_KEY;
use crate::utils::pckg;
use crate::utils::state::get_core_sub_state_for_command;
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
        pckg::concat(&self.package, "SetError")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["set_error".to_string()]
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
        line: usize,
        _env: &mut Env,
    ) -> CommandResult {
        if !arguments.is_empty() {
            let error = arguments[0].clone();

            let sub_state = get_core_sub_state_for_command(state, STATE_KEY.to_string());
            sub_state.insert("error".to_string(), StateValue::String(error));
            sub_state.insert("line".to_string(), StateValue::String(line.to_string()));
            sub_state.remove("source");

            CommandResult::Continue(None)
        } else {
            CommandResult::Error("Invalid input provided.".to_string())
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
