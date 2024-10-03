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
        pckg::concat(&self.package, "UnsetAllVars")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["unset_all_vars".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.len() > 1 && arguments.args[0] == "--prefix" {
            let prefix = &arguments.args[1];

            arguments
                .variables
                .retain(|key, _| !key.starts_with(prefix));
        } else {
            arguments.variables.clear();
        }

        CommandResult::Continue(None)
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
