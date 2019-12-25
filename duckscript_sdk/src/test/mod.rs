use duckscript::runner;
use duckscript::types::command::Command;
use duckscript::types::error::ScriptError;
use duckscript::types::runtime::Context;

pub(crate) fn test_common_command_functions(command: Box<dyn Command>) {
    assert!(command.name().len() > 0);
    assert!(command.help().len() > 0);
    command.aliases();
}

fn run_command(command: Box<dyn Command>, script: &str) -> Result<Context, ScriptError> {
    let mut context = Context::new();
    context.commands.set(command)?;
    runner::run_script(script, context)
}

pub(crate) fn run_command_and_fail(command: Box<dyn Command>, script: &str) {
    let result = run_command(command, script);
    assert!(result.is_err());
}

pub(crate) fn validate_command(
    command: Box<dyn Command>,
    script: &str,
    output_key: Option<String>,
    output_value: Option<String>,
    contains_value: bool,
) -> Context {
    let result = run_command(command, script);
    match result {
        Ok(context) => {
            match output_key {
                Some(ref key) => {
                    let value_option = context.variables.get(key);
                    assert!(value_option.is_some());

                    let value = value_option.unwrap();

                    if contains_value {
                        assert!(value.contains(&output_value.unwrap()));
                    } else {
                        assert_eq!(value, &output_value.unwrap());
                    }
                }
                None => assert!(context.variables.is_empty()),
            };

            context
        }
        Err(error) => panic!(error.to_string()),
    }
}
