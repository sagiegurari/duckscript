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
        pckg::concat(&self.package, "GetByName")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["get_by_name".to_string()]
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
        } else {
            match arguments.variables.get(&arguments.args[0]) {
                Some(ref value) => Some(value.to_string()),
                None => None,
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
