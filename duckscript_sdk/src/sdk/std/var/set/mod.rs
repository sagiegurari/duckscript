use crate::utils::{condition, pckg};
use duckscript::types::command::{Command, CommandArgs, CommandResult};

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

fn get_output(arguments: &Vec<String>) -> Result<Option<String>, String> {
    let mut looking_for_value = true;
    let mut last_value = None;
    for argument in arguments {
        if looking_for_value {
            last_value = Some(argument.clone());

            if condition::is_true(Some(argument.clone())) {
                return Ok(last_value);
            }

            looking_for_value = false;
        } else if argument == "or" {
            looking_for_value = true;
        } else {
            return Err(format!("Keyword 'or' expected, found: {}", &argument).to_string());
        }
    }

    if looking_for_value {
        Err("Keyword 'or' found, expected a value afterwards.".to_string())
    } else {
        Ok(last_value)
    }
}

#[derive(Clone)]
pub(crate) struct CommandImpl {
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

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        let output = if arguments.args.is_empty() {
            None
        } else if arguments.args.len() == 1 {
            Some(arguments.args[0].clone())
        } else {
            match get_output(&arguments) {
                Ok(output) => output,
                Err(error) => return CommandResult::Error(error),
            }
        };

        CommandResult::Continue(output)
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
