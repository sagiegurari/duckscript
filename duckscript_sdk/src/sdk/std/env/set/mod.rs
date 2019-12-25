use crate::utils::pckg;
use duckscript::types::command::{Command, CommandResult};
use duckscript::types::instruction::InstructionMetaInfo;
use std::env;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Set")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["set_env".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn run(&self, arguments: Vec<String>, meta_info: InstructionMetaInfo) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error(
                "Missing environment variable name and value.".to_string(),
                meta_info,
            )
        } else if arguments.len() == 1 {
            CommandResult::Error("Missing environment variable value.".to_string(), meta_info)
        } else {
            env::set_var(&arguments[0], &arguments[1]);

            CommandResult::Continue(None)
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
