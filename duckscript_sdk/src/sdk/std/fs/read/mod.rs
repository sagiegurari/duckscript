use crate::utils::io;
use crate::utils::pckg;
use duckscript::types::command::{Command, CommandResult};
use duckscript::types::instruction::InstructionMetaInfo;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Read")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["readfile".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn run(&self, arguments: Vec<String>, meta_info: InstructionMetaInfo) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("File name not provided.".to_string(), meta_info)
        } else {
            let result = io::read_text_file(&arguments[0]);

            match result {
                Ok(text) => CommandResult::Continue(Some(text)),
                Err(error) => CommandResult::Error(error.to_string(), meta_info),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
