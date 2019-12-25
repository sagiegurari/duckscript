use crate::utils::io;
use duckscript::types::command::{Command, CommandResult};
use duckscript::types::instruction::InstructionMetaInfo;
use duckscript::types::runtime::Context;
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        format!("{}::Print", &self.package).to_string()
    }

    fn aliases(&self) -> Vec<String> {
        vec!["cat".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn run(
        &self,
        _context: Rc<RefCell<&Context>>,
        arguments: Vec<String>,
        meta_info: &InstructionMetaInfo,
    ) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("File name not provided.".to_string(), meta_info.clone())
        } else {
            let result = io::read_text_file(&arguments[0]);

            match result {
                Ok(text) => {
                    println!("{}", &text);

                    CommandResult::Continue(Some(text))
                }
                Err(error) => CommandResult::Error(error.to_string(), meta_info.clone()),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
