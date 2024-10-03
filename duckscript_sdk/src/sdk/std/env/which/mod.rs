use crate::utils::pckg;
use duckscript::types::command::{Command, CommandArgs, CommandResult};
use fsio::path::from_path::FromPath;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "FindExecutable")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["which".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.is_empty() {
            CommandResult::Error("No executable name provided.".to_string())
        } else {
            match which::which(&arguments.args[0]) {
                Ok(path) => {
                    let path_string: String = FromPath::from_path(&path);
                    CommandResult::Continue(Some(path_string))
                }
                _ => CommandResult::Continue(None),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
