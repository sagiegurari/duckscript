use crate::utils::{pckg, scope};
use duckscript::types::command::{Command, CommandArgs, CommandResult, Commands};
use duckscript::types::env::Env;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "PushStack")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["scope_push_stack".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        let copy = if arguments.args.is_empty() {
            &[]
        } else if arguments.args[0] == "--copy" {
            &arguments.args[1..]
        } else {
            &[]
        };

        match scope::push(arguments.variables, arguments.state, &copy) {
            Ok(_) => CommandResult::Continue(Some("true".to_string())),
            Err(error) => CommandResult::Error(error),
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
