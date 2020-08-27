use crate::utils::pckg;
use duckscript::types::command::{Command, CommandResult};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Text")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["random_text".to_string(), "rand_text".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let length = if arguments.is_empty() {
            1
        } else {
            match arguments[0].parse() {
                Ok(value) => {
                    let value_usize: usize = value;
                    value_usize
                }
                Err(_) => {
                    return CommandResult::Error(
                        format!("Invalid length provided: {}", &arguments[0]).to_string(),
                    )
                }
            }
        };

        let mut rng = thread_rng();
        let random_value: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(length)
            .collect();

        CommandResult::Continue(Some(random_value))
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
