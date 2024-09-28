use crate::utils::pckg;
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
        pckg::concat(&self.package, "DumpInstructions")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["dump_instructions".to_string()]
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
        _arguments: Vec<String>,
        _state: &mut HashMap<String, StateValue>,
        _variables: &mut HashMap<String, String>,
        output_variable: Option<String>,
        instructions: &Vec<Instruction>,
        _commands: &mut Commands,
        _line: usize,
        env: &mut Env,
    ) -> CommandResult {
        let string_value = format!("{:#?}", instructions).to_string();

        if output_variable.is_none() {
            match writeln!(env.out, "{}", string_value) {
                Ok(_) => (),
                Err(error) => return CommandResult::Error(error.to_string()),
            }
        };

        CommandResult::Continue(Some(string_value))
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
