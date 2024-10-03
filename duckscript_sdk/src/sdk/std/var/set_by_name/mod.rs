use crate::utils::pckg;
use duckscript::types::command::{Command, CommandArgs, CommandResult};

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}
impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "SetByName")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["set_by_name".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.is_empty() {
            CommandResult::Error("Missing variable name.".to_string())
        } else {
            let output = if arguments.args.len() > 1 {
                arguments
                    .variables
                    .insert(arguments.args[0].clone(), arguments.args[1].clone());
                Some(arguments.args[1].clone())
            } else {
                arguments.variables.remove(&arguments.args[0]);
                None
            };

            CommandResult::Continue(output)
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
