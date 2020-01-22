use crate::types::command::create_alias_command;
use crate::utils::pckg;
use duckscript::types::command::Command;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    let name = pckg::concat(package, "ArrayIsEmpty");
    Box::new(create_alias_command(
        name,
        vec!["array_is_empty".to_string()],
        include_str!("help.md").to_string(),
        include_str!("script.ds").to_string(),
        1,
    ))
}
