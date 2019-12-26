use duckscript::runner;
use duckscript::types::command::Command;
use duckscript::types::error::ScriptError;
use duckscript::types::runtime::Context;

pub(crate) enum CommandValidation {
    None,
    Match(String, String),
    Contains(String, String),
}

pub(crate) fn test_common_command_functions(command: Box<dyn Command>) {
    assert!(command.name().len() > 0);
    command.help();
    command.aliases();
}

fn run_command(commands: Vec<Box<dyn Command>>, script: &str) -> Result<Context, ScriptError> {
    let mut context = Context::new();
    for command in commands {
        context.commands.set(command)?;
    }
    runner::run_script(script, context)
}

pub(crate) fn run_script_and_fail(commands: Vec<Box<dyn Command>>, script: &str) {
    let result = run_command(commands, script);
    assert!(result.is_err());
}

pub(crate) fn run_script_and_validate(
    commands: Vec<Box<dyn Command>>,
    script: &str,
    validation: CommandValidation,
) -> Context {
    let result = run_command(commands, script);
    match result {
        Ok(context) => {
            match validation {
                CommandValidation::None => assert!(context.variables.is_empty()),
                CommandValidation::Match(key, value) => {
                    assert_eq!(context.variables.get(&key), Some(&value))
                }
                CommandValidation::Contains(key, value) => {
                    assert!(context.variables.get(&key).unwrap().contains(&value))
                }
            };

            context
        }
        Err(error) => panic!(error.to_string()),
    }
}
