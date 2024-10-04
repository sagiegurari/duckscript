use crate::utils::pckg;
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
        pckg::concat(&self.package, "DumpState")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["dump_state".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        let string_value = format!("{:#?}", context.state).to_string();

        if context.output_variable.is_none() {
            match writeln!(context.env.out, "{}", &string_value) {
                Ok(_) => (),
                Err(error) => return CommandResult::Error(error.to_string()),
            };
        }

        CommandResult::Continue(Some(string_value))
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
