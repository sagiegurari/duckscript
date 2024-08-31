use crate::utils::pckg;
use crate::utils::state::{get_handles_sub_state, mutate_map};
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
        pckg::concat(&self.package, "MapPut")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["map_put".to_string(), "map_add".to_string()]
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
            CommandResult::Error("Map handle not provided.".to_string())
        } else if arguments.len() < 3 {
            CommandResult::Error("Key/Value not provided.".to_string())
        } else {
            let state = get_handles_sub_state(state);

            let key = arguments[0].clone();

            let result = mutate_map(key, state, |map| {
                let item_key = arguments[1].clone();
                let value = arguments[2].clone();

                map.insert(item_key, StateValue::String(value));

                Ok(None)
            });

            match result {
                Ok(_) => CommandResult::Continue(Some("true".to_string())),
                Err(error) => CommandResult::Error(error),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
