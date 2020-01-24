use crate::utils::pckg;
use duckscript::types::command::{Command, CommandResult};
use hostname;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Hostname")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["hostname".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, _arguments: Vec<String>) -> CommandResult {
        match hostname::get() {
            Ok(value) => CommandResult::Continue(Some(value.to_string_lossy().into_owned())),
            Err(_) => CommandResult::Continue(None),
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
