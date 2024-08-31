use crate::sdk::std::lib::alias::ALIAS_STATE_KEY;
use crate::utils::state::get_sub_state;
use crate::utils::{eval, pckg};
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::env::Env;
use duckscript::types::instruction::Instruction;
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

        fn requires_context(&self) -> bool {
            true
        }

        fn run_with_context(
            &self,
            arguments: Vec<String>,
            state: &mut HashMap<String, StateValue>,
            variables: &mut HashMap<String, String>,
            _output_variable: Option<String>,
            _instructions: &Vec<Instruction>,
            commands: &mut Commands,
            _line: usize,
            env: &mut Env,
        ) -> CommandResult {
            let mut all_arguments = vec![];
            all_arguments.append(&mut self.arguments.clone());
            all_arguments.append(&mut arguments.clone());

            eval::eval_with_error(&all_arguments, state, variables, commands, env)
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

    fn requires_context(&self) -> bool {
        true
    }

    fn run_with_context(
        &self,
        arguments: Vec<String>,
        state: &mut HashMap<String, StateValue>,
        _variables: &mut HashMap<String, String>,
        _output_variable: Option<String>,
        _instructions: &Vec<Instruction>,
        commands: &mut Commands,
        _line: usize,
        _env: &mut Env,
    ) -> CommandResult {
        if arguments.len() < 2 {
            CommandResult::Error("Invalid alias provided.".to_string())
        } else {
            let name = arguments[0].clone();

            let sub_state = get_sub_state(ALIAS_STATE_KEY.to_string(), state);

            match create_alias_command(name, arguments[1..].to_vec(), commands, sub_state) {
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
