use crate::utils::pckg;
use crate::utils::state::put_handle;
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::env::Env;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use fsio::path::from_path::FromPath;
use ignore::WalkBuilder;
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
        pckg::concat(&self.package, "GitIgnorePathArray")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["gitignore_path_array".to_string()]
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
            CommandResult::Error("Root directory not provided.".to_string())
        } else {
            let mut array = vec![];

            let (path_index, include_hidden) =
                if arguments.len() > 1 && arguments[0] == "--include-hidden" {
                    (1, true)
                } else {
                    (0, false)
                };

            for entry in WalkBuilder::new(&arguments[path_index])
                .hidden(!include_hidden)
                .parents(true)
                .git_ignore(true)
                .git_exclude(true)
                .build()
            {
                match entry {
                    Ok(path) => {
                        let value_string: String = FromPath::from_path(&path.path());
                        let state_value = StateValue::String(value_string);

                        array.push(state_value);
                    }
                    Err(error) => return CommandResult::Error(error.to_string()),
                }
            }

            let key = put_handle(state, StateValue::List(array));

            CommandResult::Continue(Some(key))
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
