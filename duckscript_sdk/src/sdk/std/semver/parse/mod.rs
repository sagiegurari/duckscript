use crate::utils::pckg;
use duckscript::types::command::{Command, CommandInvocationContext, CommandResult};
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
        pckg::concat(&self.package, "Parse")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["semver_parse".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        if context.arguments.is_empty() {
            CommandResult::Error("No semver value provided.".to_string())
        } else {
            match Version::parse(&context.arguments[0]) {
                Ok(version) => match context.output_variable {
                    Some(name) => {
                        context.variables.insert(
                            format!("{}.major", &name).to_string(),
                            version.major.to_string(),
                        );
                        context.variables.insert(
                            format!("{}.minor", &name).to_string(),
                            version.minor.to_string(),
                        );
                        context.variables.insert(
                            format!("{}.patch", &name).to_string(),
                            version.patch.to_string(),
                        );

                        CommandResult::Continue(None)
                    }
                    None => CommandResult::Continue(None),
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
