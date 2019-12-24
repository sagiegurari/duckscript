use duckscript::runner;
use duckscript::types::command::Command;
use duckscript::types::error::ScriptError;
use duckscript::types::runtime::Context;

pub(crate) fn test_common_command_functions(command: Box<dyn Command>) {
    assert!(command.name().len() > 0);
    assert!(command.help().len() > 0);
    command.aliases();
}

fn run_command(
    command: Box<dyn Command>,
    mut context: Context,
    script: &str,
) -> Result<Context, ScriptError> {
    context.commands.set(command)?;
    runner::run_script(script, context)
}

fn run_command_with_default_context(
    command: Box<dyn Command>,
    script: &str,
) -> Result<Context, ScriptError> {
    let context = Context::new();

    run_command(command, context, script)
}

pub(crate) fn run_command_and_fail_with_default_context(command: Box<dyn Command>, script: &str) {
    let result = run_command_with_default_context(command, script);
    assert!(result.is_err());
}

pub(crate) fn run_command_valid_with_default_context(
    command: Box<dyn Command>,
    script: &str,
) -> Context {
    let result = run_command_with_default_context(command, script);
    match result {
        Ok(context) => context,
        Err(error) => panic!(error.to_string()),
    }
}
