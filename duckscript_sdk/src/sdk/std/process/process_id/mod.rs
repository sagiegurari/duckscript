use crate::utils::pckg;
use duckscript::types::command::{Command, CommandArgs, CommandResult};
use std::process;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "ProcessID")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["pid".to_string(), "process_id".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, _arguments: CommandArgs) -> CommandResult {
        let id = process::id();
        CommandResult::Continue(Some(id.to_string()))
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
