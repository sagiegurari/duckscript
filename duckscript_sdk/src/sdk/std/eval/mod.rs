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
        pckg::concat(&self.package, "Eval")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["eval".to_string()]
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
        variables: &mut HashMap<String, String>,
        _output_variable: Option<String>,
        _instructions: &Vec<Instruction>,
        commands: &mut Commands,
        _test_setline: usize,
    ) -> CommandResult {
        eval::eval(&arguments, state, variables, commands)
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
