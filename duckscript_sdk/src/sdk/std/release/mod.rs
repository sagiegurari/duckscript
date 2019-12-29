use crate::utils::pckg;
use crate::utils::state::get_handles_sub_state;
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::instruction::{Instruction, InstructionMetaInfo};
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
        pckg::concat(&self.package, "Release")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["release".to_string()]
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
        _commands: &mut Commands,
        _meta_info: InstructionMetaInfo,
        _line: usize,
    ) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Continue(Some("false".to_string()))
        } else {
            let state = get_handles_sub_state(state);

            match state.remove(&arguments[0]) {
                Some(_) => CommandResult::Continue(Some("true".to_string())),
                None => CommandResult::Continue(Some("false".to_string())),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
