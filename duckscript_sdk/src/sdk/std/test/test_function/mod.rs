use crate::utils::{eval, pckg};
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
        pckg::concat(&self.package, "TestFunction")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["test_function".to_string()]
    }

    fn help(&self) -> String {
        "".to_string()
    }

    fn requires_context(&self) -> bool {
        true
    }

    fn run_with_context(
        &self,
        arguments: Vec<String>,
        _state: &mut HashMap<String, StateValue>,
        _variables: &mut HashMap<String, String>,
        _output_variable: Option<String>,
        _instructions: &Vec<Instruction>,
        commands: &mut Commands,
        _line: usize,
    ) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Crash("Function name not provided.".to_string())
        } else {
            let function_name = arguments[0].clone();

            match eval::eval(
                &vec![function_name],
                &mut HashMap::new(),
                &mut HashMap::new(),
                commands,
            ) {
                CommandResult::Crash(error) => CommandResult::Crash(error),
                _ => CommandResult::Continue(Some("true".to_string())),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
