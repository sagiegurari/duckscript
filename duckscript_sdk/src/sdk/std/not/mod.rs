use crate::utils::{condition, pckg};
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
        pckg::concat(&self.package, "Not")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["not".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.is_empty() {
            CommandResult::Error("Missing condition".to_string())
        } else {
            match condition::eval_condition(
                arguments.args,
                arguments.instructions,
                arguments.state,
                arguments.variables,
                arguments.commands,
                arguments.env,
            ) {
                Ok(passed) => {
                    let output = !passed;
                    CommandResult::Continue(Some(output.to_string()))
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
