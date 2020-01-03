use crate::utils::pckg;
use duckscript::types::command::{Command, CommandResult};
use std::env;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "PrintCurrentDirectory")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["pwd".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn run(&self, _arguments: Vec<String>) -> CommandResult {
        match env::current_dir() {
            Ok(directory_path) => {
                let directory = directory_path.display();
                println!("{}", &directory);
                CommandResult::Continue(Some(directory.to_string()))
            }
            Err(_) => CommandResult::Continue(None),
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
