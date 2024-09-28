use crate::utils::state::{get_handles_sub_state, put_handle};
use duckscript::runner;
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::env::Env;
use duckscript::types::error::ScriptError;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::{Context, StateValue};
use std::collections::HashMap;
use std::env;

#[derive(Clone)]
pub(crate) struct EmptyCommand {}

impl Command for EmptyCommand {
    fn name(&self) -> String {
        "test_empty".to_string()
    }

    fn aliases(&self) -> Vec<String> {
        vec!["test_empty1".to_string(), "test_empty2".to_string()]
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, _arguments: Vec<String>) -> CommandResult {
        CommandResult::Continue(None)
    }
}

#[derive(Clone)]
pub(crate) struct ErrorCommand {}

impl Command for ErrorCommand {
    fn name(&self) -> String {
        "test_error".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, _arguments: Vec<String>) -> CommandResult {
        CommandResult::Error("test".to_string())
    }
}

#[derive(Clone)]
pub(crate) struct SetCommand {}

impl Command for SetCommand {
    fn name(&self) -> String {
        "test_set".to_string()
    }

    fn help(&self) -> String {
        "".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Continue(None)
        } else {
            CommandResult::Continue(Some(arguments[0].clone()))
        }
    }
}

#[derive(Clone)]
pub(crate) struct SetHandleCommand {}

impl Command for SetHandleCommand {
    fn name(&self) -> String {
        "test_set_handle".to_string()
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
        _commands: &mut Commands,
        _line: usize,
        _env: &mut Env,
    ) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Continue(None)
        } else {
            let state = get_handles_sub_state(state);
            state.insert(arguments[0].clone(), StateValue::String("test".to_string()));
            CommandResult::Continue(None)
        }
    }
}

#[derive(Clone)]
pub(crate) struct ArrayCommand {}

impl Command for ArrayCommand {
    fn name(&self) -> String {
        "test_array".to_string()
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
        _commands: &mut Commands,
        _line: usize,
        _env: &mut Env,
    ) -> CommandResult {
        let mut array = vec![];

        for argument in arguments {
            array.push(StateValue::String(argument));
        }

        let key = put_handle(state, StateValue::List(array));

        CommandResult::Continue(Some(key))
    }
}

#[derive(Clone)]
pub(crate) struct OnErrorCommand {}

impl Command for OnErrorCommand {
    fn name(&self) -> String {
        "on_error".to_string()
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
        _state: &mut HashMap<String, StateValue>,
        variables: &mut HashMap<String, String>,
        _output_variable: Option<String>,
        _instructions: &Vec<Instruction>,
        _commands: &mut Commands,
        _line: usize,
        _env: &mut Env,
    ) -> CommandResult {
        println!("on error: {:#?}", &arguments);

        let mut index = 0;
        for argument in arguments {
            index = index + 1;
            variables.insert(index.to_string(), argument.clone());
        }

        variables.insert("on_error_invoked".to_string(), "true".to_string());

        CommandResult::Continue(None)
    }
}

pub(crate) enum CommandValidation {
    None,
    PositiveNumber(String),
    StringLength(String, usize),
    Match(String, String),
    Contains(String, String),
    Any(String, Vec<String>),
    Ignore,
    Undefined(String),
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

    if !context.commands.exists("on_error") {
        let added = context.commands.set(Box::new(OnErrorCommand {}));
        assert!(added.is_ok());
    }

    runner::run_script(script, context, None)
}

pub(crate) fn run_script_and_crash(commands: Vec<Box<dyn Command>>, script: &str) {
    let result = run_command(commands, script);
    assert!(result.is_err());
}

pub(crate) fn run_script_and_error(
    commands: Vec<Box<dyn Command>>,
    script: &str,
    output_variable: &str,
) -> Context {
    let result = run_command(commands, script);
    match result {
        Ok(context) => {
            if !output_variable.is_empty() {
                assert_eq!(
                    context.variables.get(&output_variable.to_string()).unwrap(),
                    "false"
                );
            }

            assert_eq!(
                context
                    .variables
                    .get(&"on_error_invoked".to_string())
                    .unwrap(),
                "true"
            );

            context
        }
        Err(error) => panic!("{}", error.to_string()),
    }
}

pub(crate) fn run_script_and_validate(
    commands: Vec<Box<dyn Command>>,
    script: &str,
    validation: CommandValidation,
) -> Context {
    let result = run_command(commands, script);
    match result {
        Ok(context) => {
            assert!(context
                .variables
                .get(&"on_error_invoked".to_string())
                .is_none());

            match validation {
                CommandValidation::None => assert!(context.variables.is_empty()),
                CommandValidation::Match(key, value) => {
                    assert!(!context.variables.is_empty());
                    assert_eq!(context.variables.get(&key), Some(&value))
                }
                CommandValidation::Contains(key, value) => {
                    assert!(!context.variables.is_empty());

                    let var_value = context.variables.get(&key).unwrap();
                    assert!(
                        var_value.contains(&value),
                        "The value: {} is not contained in: {}",
                        &value,
                        &var_value
                    )
                }
                CommandValidation::Any(key, values) => {
                    assert!(!context.variables.is_empty());

                    let var_value = context.variables.get(&key).unwrap();
                    assert!(
                        values.contains(&var_value),
                        "The value: {} is not contained in: {:#?}",
                        &var_value,
                        &values
                    )
                }
                CommandValidation::PositiveNumber(key) => {
                    assert!(!context.variables.is_empty());

                    let var_value = context.variables.get(&key).unwrap();
                    let numeric_value: u128 = var_value.parse().unwrap();
                    assert!(numeric_value > 0)
                }
                CommandValidation::StringLength(key, length) => {
                    assert!(!context.variables.is_empty());

                    let var_value = context.variables.get(&key).unwrap();
                    assert!(var_value.len() == length)
                }
                CommandValidation::Ignore => {
                    assert!(!context.variables.is_empty());
                }
                CommandValidation::Undefined(key) => {
                    assert!(!context.variables.is_empty());
                    assert!(!context.variables.contains_key(&key));
                }
            };

            context
        }
        Err(error) => panic!("{}", error.to_string()),
    }
}

pub(crate) fn is_handles_empty(state: &HashMap<String, StateValue>) {
    let handles_state = state.get("handles").unwrap();

    match handles_state {
        StateValue::SubState(state) => assert!(state.is_empty()),
        _ => panic!("Invalid state type."),
    }
}

pub(crate) fn skip_unstable() -> bool {
    let skip = match env::var("CARGO_MAKE_DUCKSCRIPT_SKIP_UNSTABLE_TESTS") {
        Ok(value) => value == "true",
        Err(_) => false,
    };

    if skip {
        println!("Skipping Test...");
    }

    skip
}
