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
        pckg::concat(&self.package, "Set")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["set".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn run(&self, arguments: Vec<String>, _meta_info: InstructionMetaInfo) -> CommandResult {
        let output = if arguments.is_empty() {
            None
        } else {
            Some(arguments[0].clone())
        };

        CommandResult::Continue(output)
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
