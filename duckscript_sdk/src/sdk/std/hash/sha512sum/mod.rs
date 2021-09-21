use crate::types::command::create_alias_command;
use crate::utils::pckg;
use duckscript::types::command::Command;
use duckscript::types::error::ScriptError;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

pub(crate) fn create(package: &str) -> Result<Box<dyn Command>, ScriptError> {
    let name = pckg::concat(package, "Sha512Sum");
    let command = create_alias_command(
        name,
        vec!["sha512sum".to_string(), "sha512sum".to_string()],
        include_str!("help.md").to_string(),
        "sha512sum".to_string(),
        include_str!("script.ds").to_string(),
        1,
    )?;

    Ok(Box::new(command))
}
