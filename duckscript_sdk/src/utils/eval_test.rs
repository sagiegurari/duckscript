use super::*;
use crate::test::SetCommand;

#[test]
fn eval_empty_arguments() {
    let mut commands = Commands::new();

    let result = eval(
        &vec![],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut commands,
    );

    match result {
        CommandResult::Continue(value) => assert!(value.is_none()),
        _ => panic!("invalid result type."),
    };
}

#[test]
fn eval_label_only() {
    let mut commands = Commands::new();

    let result = eval(
        &vec![":label".to_string()],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut commands,
    );

    match result {
        CommandResult::Continue(value) => assert!(value.is_none()),
        _ => panic!("invalid result type."),
    };
}

#[test]
fn eval_command_no_output() {
    let mut commands = Commands::new();
    match commands.set(Box::new(SetCommand {})) {
        Err(error) => panic!("{}", error),
        _ => (),
    };

    let result = eval(
        &vec!["test_set".to_string()],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut commands,
    );

    match result {
        CommandResult::Continue(value) => assert!(value.is_none()),
        _ => panic!("invalid result type."),
    };
}

#[test]
fn eval_command_with_output() {
    let mut commands = Commands::new();
    match commands.set(Box::new(SetCommand {})) {
        Err(error) => panic!("{}", error),
        _ => (),
    };

    let result = eval(
        &vec!["test_set".to_string(), "test".to_string()],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut commands,
    );

    match result {
        CommandResult::Continue(value) => assert_eq!(value.unwrap(), "test"),
        _ => panic!("invalid result type."),
    };
}

#[test]
fn eval_command_with_output_with_spaces() {
    let mut commands = Commands::new();
    match commands.set(Box::new(SetCommand {})) {
        Err(error) => panic!("{}", error),
        _ => (),
    };

    let result = eval(
        &vec!["test_set".to_string(), "test 1 2 3".to_string()],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut commands,
    );

    match result {
        CommandResult::Continue(value) => assert_eq!(value.unwrap(), "test 1 2 3"),
        _ => panic!("invalid result type."),
    };
}

#[test]
fn eval_command_with_output_with_spaces_and_all_line_types() {
    let mut commands = Commands::new();
    match commands.set(Box::new(SetCommand {})) {
        Err(error) => panic!("{}", error),
        _ => (),
    };

    let result = eval(
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
    );

    match result {
        CommandResult::Continue(value) => assert_eq!(value.unwrap(), "test 1 2 3"),
        _ => panic!("invalid result type."),
    };
}

#[test]
fn eval_parse_error() {
    let mut commands = Commands::new();

    let result = eval(
        &vec![":label".to_string(), ":label".to_string()],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut commands,
    );

    match result {
        CommandResult::Error(_) => (),
        _ => panic!("invalid result type."),
    };
}
