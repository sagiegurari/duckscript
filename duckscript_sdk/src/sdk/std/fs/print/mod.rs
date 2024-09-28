use crate::utils::{io, pckg};
use duckscript::types::command::{Command, CommandArgs, CommandResult, Commands};
use duckscript::types::env::Env;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Print")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["cat".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.is_empty() {
            CommandResult::Error("File name not provided.".to_string())
        } else {
            let mut all_text = String::new();
            for argument in &arguments.args {
                let result = io::read_text_file(&argument);

                match result {
                    Ok(text) => all_text.push_str(&text),
                    Err(error) => return CommandResult::Error(error.to_string()),
                }
            }

            match writeln!(arguments.env.out, "{}", &all_text) {
                Ok(_) => CommandResult::Continue(Some(all_text)),
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
