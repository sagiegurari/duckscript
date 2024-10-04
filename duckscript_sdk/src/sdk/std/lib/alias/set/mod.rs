use crate::sdk::std::lib::alias::ALIAS_STATE_KEY;
use crate::utils::state::get_sub_state;
use crate::utils::{eval, pckg};
use duckscript::types::command::{Command, CommandInvocationContext, CommandResult, Commands};
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

fn create_alias_command(
    name: String,
    arguments: Vec<String>,
    commands: &mut Commands,
    sub_state: &mut HashMap<String, StateValue>,
) -> Result<(), String> {
    #[derive(Clone)]
    struct AliasCommand {
        name: String,
        arguments: Vec<String>,
    }

    impl Command for AliasCommand {
        fn name(&self) -> String {
            self.name.clone()
        }

        fn help(&self) -> String {
            "".to_string()
        }

        fn clone_and_box(&self) -> Box<dyn Command> {
            Box::new((*self).clone())
        }

        fn run(&self, context: CommandInvocationContext) -> CommandResult {
            let mut all_arguments = vec![];
            all_arguments.append(&mut self.arguments.clone());
            all_arguments.append(&mut context.arguments.clone());

            eval::eval_with_error(
                &all_arguments,
                context.state,
                context.variables,
                context.commands,
                context.env,
            )
        }
    }

    let command = AliasCommand {
        name: name.clone(),
        arguments,
    };

    match commands.set(Box::new(command)) {
        Ok(_) => {
            sub_state.insert(name.clone(), StateValue::Boolean(true));
            Ok(())
        }
        Err(error) => Err(error.to_string()),
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
        vec!["alias".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        if context.arguments.len() < 2 {
            CommandResult::Error("Invalid alias provided.".to_string())
        } else {
            let name = context.arguments[0].clone();

            let sub_state = get_sub_state(ALIAS_STATE_KEY.to_string(), context.state);

            match create_alias_command(
                name,
                context.arguments[1..].to_vec(),
                context.commands,
                sub_state,
            ) {
                Ok(_) => CommandResult::Continue(Some("true".to_string())),
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
