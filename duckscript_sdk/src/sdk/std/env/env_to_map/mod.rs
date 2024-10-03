use crate::utils::pckg;
use crate::utils::state::put_handle;
use duckscript::types::command::{Command, CommandArgs, CommandResult};
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;
use std::env;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "EnvToMap")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["env_to_map".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        let all_vars = env::vars();
        let mut map = HashMap::new();

        for (var_key, var_value) in all_vars {
            map.insert(var_key, StateValue::String(var_value.to_string()));
        }

        let key = put_handle(arguments.state, StateValue::SubState(map));

        CommandResult::Continue(Some(key))
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
