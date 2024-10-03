use crate::utils::pckg;
use duckscript::types::command::{Command, CommandArgs, CommandResult};

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

fn parse_number(string_value: &str) -> Result<isize, String> {
    match string_value.parse() {
        Ok(value) => Ok(value),
        Err(_) => Err(format!("Non numeric value: {} provided.", string_value).to_string()),
    }
}

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "SubString")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["substring".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.is_empty() {
            CommandResult::Error("No arguments provided.".to_string())
        } else {
            let string_value = arguments.args[0].clone();
            let string_len = string_value.len() as isize;

            let (start, end) = if arguments.args.len() == 1 {
                (0, string_len)
            } else if arguments.args.len() == 2 {
                match parse_number(&arguments.args[1]) {
                    Ok(value) => {
                        if value >= 0 {
                            if value > (string_len - 1) {
                                return CommandResult::Error(
                                    "Start index cannot be bigger than total text size."
                                        .to_string(),
                                );
                            } else {
                                (value, string_len)
                            }
                        } else {
                            let end_index = string_len + value;

                            if end_index < 0 {
                                return CommandResult::Error(
                                    "Index from end cannot be bigger than total text size."
                                        .to_string(),
                                );
                            } else {
                                (0, end_index)
                            }
                        }
                    }
                    Err(error) => return CommandResult::Error(error.to_string()),
                }
            } else {
                let start = match parse_number(&arguments.args[1]) {
                    Ok(value) => {
                        if value > (string_len - 1) {
                            return CommandResult::Error(
                                "Start index cannot be bigger than total text size.".to_string(),
                            );
                        } else {
                            value
                        }
                    }
                    Err(error) => return CommandResult::Error(error.to_string()),
                };
                let end = match parse_number(&arguments.args[2]) {
                    Ok(value) => {
                        if value >= start {
                            if value > (string_len - 1) {
                                return CommandResult::Error(
                                    "End index cannot be bigger than total text size.".to_string(),
                                );
                            } else {
                                value
                            }
                        } else {
                            return CommandResult::Error(
                                "End index cannot be smaller than start index.".to_string(),
                            );
                        }
                    }
                    Err(error) => return CommandResult::Error(error.to_string()),
                };

                (start, end)
            };

            let start_index: usize = start.try_into().unwrap();
            let end_index: usize = end.try_into().unwrap();

            let sub_string = &string_value.as_str()[start_index..end_index];

            CommandResult::Continue(Some(sub_string.to_string()))
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
