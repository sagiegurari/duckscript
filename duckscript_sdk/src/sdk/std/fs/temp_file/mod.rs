use crate::utils::{io, pckg};
use duckscript::types::command::{Command, CommandResult};
use fsio::path::get_temporary_file_path;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "TempFile")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["temp_file".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let extension = if arguments.is_empty() {
            "tmp"
        } else {
            &arguments[0]
        };

        let path = get_temporary_file_path(extension);

        match io::create_empty_file(&path) {
            Ok(()) => CommandResult::Continue(Some(path)),
            Err(error) => CommandResult::Error(error),
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
