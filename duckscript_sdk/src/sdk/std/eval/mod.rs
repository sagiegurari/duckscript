use crate::utils::{eval, pckg};
use duckscript::types::command::{Command, CommandInvocationContext, CommandResult};

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Eval")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["eval".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, mut context: CommandInvocationContext) -> CommandResult {
        eval::eval_with_error(
            &context.arguments,
            &mut context.state,
            &mut context.variables,
            &mut context.commands,
            &mut context.env,
        )
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
