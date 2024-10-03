use super::*;
use crate::sdk::std::string::equals::create as EqCreate;
use crate::test::SetCommand;

#[test]
fn eval_with_error_empty_arguments() {
    let mut commands = Commands::new();
    let mut env = Env::default();

    let result = eval_with_error(
        &vec![],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut commands,
        &mut env,
    );

    match result {
        CommandResult::Continue(value) => assert!(value.is_none()),
        _ => panic!("invalid result type."),
    };
}

#[test]
fn eval_with_error_label_only() {
    let mut commands = Commands::new();
    let mut env = Env::default();

    let result = eval_with_error(
        &vec![":label".to_string()],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut commands,
        &mut env,
    );

    match result {
        CommandResult::Continue(value) => assert!(value.is_none()),
        _ => panic!("invalid result type."),
    };
}

#[test]
fn eval_with_error_command_no_output() {
    let mut commands = Commands::new();
    match commands.set(Box::new(SetCommand {})) {
        Err(error) => panic!("{}", error),
        _ => (),
    };
    let mut env = Env::default();

    let result = eval_with_error(
        &vec!["test_set".to_string()],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut commands,
        &mut env,
    );

    match result {
        CommandResult::Continue(value) => assert!(value.is_none()),
        _ => panic!("invalid result type."),
    };
}

#[test]
fn eval_with_error_command_with_output() {
    let mut commands = Commands::new();
    match commands.set(Box::new(SetCommand {})) {
        Err(error) => panic!("{}", error),
        _ => (),
    };
    let mut env = Env::default();

    let result = eval_with_error(
        &vec!["test_set".to_string(), "test".to_string()],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut commands,
        &mut env,
    );

    match result {
        CommandResult::Continue(value) => assert_eq!(value.unwrap(), "test"),
        _ => panic!("invalid result type."),
    };
}

#[test]
fn eval_with_error_command_with_output_with_spaces() {
    let mut commands = Commands::new();
    match commands.set(Box::new(SetCommand {})) {
        Err(error) => panic!("{}", error),
        _ => (),
    };
    let mut env = Env::default();

    let result = eval_with_error(
        &vec!["test_set".to_string(), "test 1 2 3".to_string()],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut commands,
        &mut env,
    );

    match result {
        CommandResult::Continue(value) => assert_eq!(value.unwrap(), "test 1 2 3"),
        _ => panic!("invalid result type."),
    };
}

#[test]
fn eval_with_error_command_with_output_with_spaces_and_all_line_types() {
    let mut commands = Commands::new();
    match commands.set(Box::new(SetCommand {})) {
        Err(error) => panic!("{}", error),
        _ => (),
    };
    let mut env = Env::default();

    let result = eval_with_error(
        &vec![
            ":label".to_string(),
            "out".to_string(),
            "=".to_string(),
            "test_set".to_string(),
            "test 1 2 3".to_string(),
            "another".to_string(),
            "# this is a comment".to_string(),
        ],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut commands,
        &mut env,
    );

    match result {
        CommandResult::Continue(value) => assert_eq!(value.unwrap(), "test 1 2 3"),
        _ => panic!("invalid result type."),
    };
}

#[test]
fn eval_with_error_parse_error() {
    let mut commands = Commands::new();
    let mut env = Env::default();

    let result = eval_with_error(
        &vec![":label".to_string(), ":label".to_string()],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut commands,
        &mut env,
    );

    match result {
        CommandResult::Error(_) => (),
        _ => panic!("invalid result type."),
    };
}

#[test]
fn eval_with_eq_empty_args() {
    let mut commands = Commands::new();
    match commands.set(EqCreate("")) {
        Err(error) => panic!("{}", error),
        _ => (),
    };
    let mut env = Env::default();

    let result = eval_with_error(
        &vec!["eq".to_string(), "".to_string(), "".to_string()],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut commands,
        &mut env,
    );

    match result {
        CommandResult::Continue(value) => assert_eq!(value.unwrap(), "true"),
        _ => panic!("invalid result type."),
    };
}

#[test]
fn eval_with_eq_true_args() {
    let mut commands = Commands::new();
    match commands.set(EqCreate("")) {
        Err(error) => panic!("{}", error),
        _ => (),
    };
    let mut env = Env::default();

    let result = eval_with_error(
        &vec!["eq".to_string(), "true".to_string(), "true".to_string()],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut commands,
        &mut env,
    );

    match result {
        CommandResult::Continue(value) => assert_eq!(value.unwrap(), "true"),
        _ => panic!("invalid result type."),
    };
}

#[test]
fn eval_with_eq_true_and_false_args() {
    let mut commands = Commands::new();
    match commands.set(EqCreate("")) {
        Err(error) => panic!("{}", error),
        _ => (),
    };
    let mut env = Env::default();

    let result = eval_with_error(
        &vec!["eq".to_string(), "true".to_string(), "false".to_string()],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut commands,
        &mut env,
    );

    match result {
        CommandResult::Continue(value) => assert_eq!(value.unwrap(), "false"),
        _ => panic!("invalid result type."),
    };
}
