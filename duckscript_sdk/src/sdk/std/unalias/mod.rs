use crate::sdk::std::alias;
use crate::utils::pckg;
use crate::utils::state::get_sub_state;
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Unalias")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["unalias".to_string()]
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
        _output_variable: Option<String>,
        _instructions: &Vec<Instruction>,
        commands: &mut Commands,
        _line: usize,
    ) -> CommandResult {
        if arguments.len() != 1 {
            CommandResult::Error("Invalid alias name provided.".to_string())
        } else {
            let sub_state = get_sub_state(alias::ALIAS_STATE_KEY.to_string(), state);

            let removed = if sub_state.contains_key(&arguments[0]) {
                commands.remove(&arguments[0]);
                sub_state.remove(&arguments[0]);

                true
            } else {
                false
            };

            CommandResult::Continue(Some(removed.to_string()))
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
