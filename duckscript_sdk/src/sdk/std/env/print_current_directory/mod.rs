use crate::utils::pckg;
use duckscript::types::command::{Command, CommandArgs, CommandResult};
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
        pckg::concat(&self.package, "PrintCurrentDirectory")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["pwd".to_string(), "print_current_directory".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        match env::current_dir() {
            Ok(directory_path) => {
                let directory = directory_path.display();
                match writeln!(arguments.env.out, "{}", &directory) {
                    Ok(_) => CommandResult::Continue(Some(directory.to_string())),
                    Err(error) => CommandResult::Error(error.to_string()),
                }
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
