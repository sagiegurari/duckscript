use crate::utils::pckg;
use duckscript::types::command::{Command, CommandArgs, CommandResult};
use fsio::path::from_path::FromPath;
use std::env;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "TempDirectory")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["temp_dir".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, _arguments: CommandArgs) -> CommandResult {
        let directory_path = env::temp_dir();
        let directory = FromPath::from_path(&directory_path);

        CommandResult::Continue(Some(directory))
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
