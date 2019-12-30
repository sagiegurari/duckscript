use crate::utils::state::get_sub_state;
use crate::utils::{eval, pckg};
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

pub(crate) static ALIAS_STATE_KEY: &str = "ALIAS_STATE";

fn create_alias_command(
    name: String,
    arguments: Vec<String>,
    commands: &mut Commands,
    sub_state: &mut HashMap<String, StateValue>,
) -> Result<(), String> {
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
            _test_setline: usize,
        ) -> CommandResult {
            let mut all_arguments = vec![];
            all_arguments.append(&mut self.arguments.clone());
            all_arguments.append(&mut arguments.clone());

            eval::eval(&all_arguments, state, variables, commands)
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

struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Alias")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["alias".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
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
        _test_setline: usize,
    ) -> CommandResult {
        if arguments.len() < 2 {
            CommandResult::Error("Invalid alias provided.".to_string())
        } else {
            let name = arguments[0].clone();

            let sub_state = get_sub_state(ALIAS_STATE_KEY.to_string(), state);

            match create_alias_command(name, arguments[1..].to_vec(), commands, sub_state) {
                Ok(_) => CommandResult::Continue(None),
                Err(error) => CommandResult::Error(error),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
