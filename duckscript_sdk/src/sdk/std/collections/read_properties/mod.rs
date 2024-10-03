use crate::utils::pckg;
use duckscript::types::command::{Command, CommandArgs, CommandResult};
use java_properties::read;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "ReadProperties")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["read_properties".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.len() < 1 {
            CommandResult::Error("Missing properties text argument.".to_string())
        } else {
            let (prefix, text) = if arguments.args.len() >= 3 && arguments.args[0] == "--prefix" {
                (arguments.args[1].to_string(), arguments.args[2].to_string())
            } else {
                ("".to_string(), arguments.args[0].to_string())
            };

            match read(text.as_bytes()) {
                Ok(data) => {
                    for (key, value) in &data {
                        let mut var_key = key.to_string();
                        if !prefix.is_empty() {
                            var_key.insert(0, '.');
                            var_key.insert_str(0, &prefix);
                        }

                        arguments.variables.insert(var_key, value.to_string());
                    }

                    CommandResult::Continue(Some(data.len().to_string()))
                }
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
