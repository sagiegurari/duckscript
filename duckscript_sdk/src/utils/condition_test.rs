use super::*;
use crate::test::{ErrorCommand, SetCommand};

#[test]
fn is_true_none() {
    let passed = is_true(None);

    assert!(!passed);
}

#[test]
fn is_true_0() {
    let passed = is_true(Some("0".to_string()));

    assert!(!passed);
}

#[test]
fn is_true_empty() {
    let passed = is_true(Some("".to_string()));

    assert!(!passed);
}

#[test]
fn is_true_no() {
    let passed = is_true(Some("no".to_string()));

    assert!(!passed);
}

#[test]
fn is_true_no_uppercase() {
    let passed = is_true(Some("NO".to_string()));

    assert!(!passed);
}

#[test]
fn is_true_false() {
    let passed = is_true(Some("false".to_string()));

    assert!(!passed);
}

#[test]
fn is_true_false_uppercase() {
    let passed = is_true(Some("FALSE".to_string()));

    assert!(!passed);
}

#[test]
fn is_true_valid() {
    let passed = is_true(Some("some value".to_string()));

    assert!(passed);
}

#[test]
fn eval_condition_empty() {
    let result = eval_condition(
        vec![],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut Commands::new(),
    );

    assert!(result.is_ok());

    let passed = result.unwrap();

    assert!(!passed);
}

#[test]
fn eval_condition_value_true() {
    let result = eval_condition(
        vec!["true".to_string()],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut Commands::new(),
    );

    assert!(result.is_ok());

    let passed = result.unwrap();

    assert!(passed);
}

#[test]
fn eval_condition_value_false() {
    let result = eval_condition(
        vec!["false".to_string()],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut Commands::new(),
    );

    assert!(result.is_ok());

    let passed = result.unwrap();

    assert!(!passed);
}

#[test]
fn eval_condition_command_true() {
    let mut commands = Commands::new();
    match commands.set(Box::new(SetCommand {})) {
        Ok(_) => (),
        _ => panic!("Test error"),
    }

    let result = eval_condition(
        vec!["test_set".to_string(), "true".to_string()],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut commands,
    );

    assert!(result.is_ok());

    let passed = result.unwrap();

    assert!(passed);
}

#[test]
fn eval_condition_command_false() {
    let mut commands = Commands::new();
    match commands.set(Box::new(SetCommand {})) {
        Ok(_) => (),
        _ => panic!("Test error"),
    }

    let result = eval_condition(
        vec!["test_set".to_string(), "false".to_string()],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut commands,
    );

    assert!(result.is_ok());

    let passed = result.unwrap();

    assert!(!passed);
}

#[test]
fn eval_condition_command_error() {
    let mut commands = Commands::new();
    match commands.set(Box::new(ErrorCommand {})) {
        Ok(_) => (),
        _ => panic!("Test error"),
    }

    let result = eval_condition(
        vec!["test_error".to_string()],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut commands,
    );

    assert!(result.is_err());
}
