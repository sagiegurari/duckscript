use crate::utils::pckg;
use duckscript::types::command::{Command, CommandResult};
use semver::Version;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "IsNewer")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["semver_is_newer".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if arguments.len() < 2 {
            CommandResult::Error("Missing semver values to compare.".to_string())
        } else {
            match Version::parse(&arguments[0]) {
                Ok(newer_version) => match Version::parse(&arguments[1]) {
                    Ok(older_version) => {
                        let result = if newer_version > older_version {
                            true
                        } else {
                            false
                        };

                        CommandResult::Continue(Some(result.to_string()))
                    }
                    Err(error) => CommandResult::Error(error.to_string()),
                },
                Err(error) => CommandResult::Error(error.to_string()),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
