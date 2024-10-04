use crate::utils::{flags, pckg};
use duckscript::types::command::{Command, CommandInvocationContext, CommandResult};
use duckscript::types::env::Env;
use fs_extra::dir::{ls, DirEntryAttr, DirEntryValue};
use fsio::path::{get_basename, get_parent_directory};
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

fn get_string_value(
    key: DirEntryAttr,
    attributes: &HashMap<DirEntryAttr, DirEntryValue>,
) -> String {
    match attributes.get(&key) {
        Some(ref entry_value) => match entry_value {
            DirEntryValue::String(value) => value.to_string(),
            _ => "".to_string(),
        },
        None => "".to_string(),
    }
}

fn get_u64_value(key: DirEntryAttr, attributes: &HashMap<DirEntryAttr, DirEntryValue>) -> String {
    match attributes.get(&key) {
        Some(ref entry_value) => match entry_value {
            DirEntryValue::U64(value) => value.to_string(),
            _ => "".to_string(),
        },
        None => "".to_string(),
    }
}

fn get_boolean_value(key: DirEntryAttr, attributes: &HashMap<DirEntryAttr, DirEntryValue>) -> bool {
    match attributes.get(&key) {
        Some(ref entry_value) => match entry_value {
            DirEntryValue::Boolean(value) => *value,
            _ => false,
        },
        None => false,
    }
}

fn print_entry(env: &mut Env, item: &HashMap<DirEntryAttr, DirEntryValue>, extended_details: bool) {
    if extended_details {
        let directory_flag = if get_boolean_value(DirEntryAttr::IsDir, &item) {
            "<DIR>"
        } else {
            ""
        };

        writeln!(
            env.out,
            "{}\t{}\t{}",
            get_u64_value(DirEntryAttr::FileSize, &item),
            directory_flag,
            get_string_value(DirEntryAttr::FullName, &item)
        )
        .unwrap();
    } else {
        writeln!(
            env.out,
            "{} ",
            get_string_value(DirEntryAttr::FullName, &item)
        )
        .unwrap();
    }
}

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "List")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["ls".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        let (path_str, flags) = if context.arguments.is_empty() {
            (".", "")
        } else if context.arguments.len() == 1 {
            if flags::is_unix_flags_argument(&context.arguments[0]) {
                (".", context.arguments[0].as_str())
            } else {
                (context.arguments[0].as_str(), "")
            }
        } else if flags::is_unix_flags_argument(&context.arguments[0]) {
            (context.arguments[1].as_str(), context.arguments[0].as_str())
        } else {
            (context.arguments[0].as_str(), "")
        };

        let path = Path::new(path_str);

        if !path.exists() {
            CommandResult::Continue(Some("false".to_string()))
        } else {
            let extended_details = flags::is_unix_flag_exists('l', flags);

            let mut config = HashSet::new();
            config.insert(DirEntryAttr::FullName);

            if extended_details {
                config.insert(DirEntryAttr::FileSize);
                config.insert(DirEntryAttr::IsDir);
            }

            let (is_file, query_path) = if path.is_file() {
                match get_parent_directory(path_str) {
                    Some(value) => (true, value),
                    None => return CommandResult::Continue(Some("false".to_string())),
                }
            } else {
                (false, path_str.to_string())
            };

            let result = ls(&query_path, &config);

            match result {
                Ok(ls_result) => {
                    let items = ls_result.items;

                    if is_file {
                        let file_name = match get_basename(path_str) {
                            Some(value) => value,
                            None => path_str.to_string(),
                        };

                        for item in items {
                            let item_name = get_string_value(DirEntryAttr::FullName, &item);

                            if item_name == file_name {
                                print_entry(context.env, &item, extended_details);
                                break;
                            }
                        }
                    } else {
                        for item in items {
                            print_entry(context.env, &item, extended_details);
                        }
                    }

                    CommandResult::Continue(Some("true".to_string()))
                }
                Err(_) => CommandResult::Continue(Some("false".to_string())),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
