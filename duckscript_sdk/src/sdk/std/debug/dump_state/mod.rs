use crate::utils::pckg;
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
        pckg::concat(&self.package, "DumpState")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["dump_state".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        let string_value = format!("{:#?}", arguments.state).to_string();

        if arguments.output_variable.is_none() {
            match writeln!(arguments.env.out, "{}", &string_value) {
                Ok(_) => (),
                Err(error) => return CommandResult::Error(error.to_string()),
            };
        }

        CommandResult::Continue(Some(string_value))
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
