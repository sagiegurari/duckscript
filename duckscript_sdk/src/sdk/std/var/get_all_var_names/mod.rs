use crate::utils::pckg;
use crate::utils::state::put_handle;
use duckscript::types::command::{Command, CommandArgs, CommandResult};
use duckscript::types::runtime::StateValue;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "GetAllVarNames")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["get_all_var_names".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        let mut array = vec![];

        for key in arguments.variables.keys() {
            array.push(StateValue::String(key.to_string()));
        }

        let key = put_handle(arguments.state, StateValue::List(array));

        CommandResult::Continue(Some(key))
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
