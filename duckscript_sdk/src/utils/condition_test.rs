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
        &vec![],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut Commands::new(),
        &mut Env::default(),
    );

    assert!(result.is_ok());

    let passed = result.unwrap();

    assert!(!passed);
}

#[test]
fn eval_condition_value_true() {
    let result = eval_condition(
        vec!["true".to_string()],
        &vec![],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut Commands::new(),
        &mut Env::default(),
    );

    assert!(result.is_ok());

    let passed = result.unwrap();

    assert!(passed);
}

#[test]
fn eval_condition_value_false() {
    let result = eval_condition(
        vec!["false".to_string()],
        &vec![],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut Commands::new(),
        &mut Env::default(),
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
        &vec![],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut commands,
        &mut Env::default(),
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
        &vec![],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut commands,
        &mut Env::default(),
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
        &vec![],
        &mut HashMap::new(),
        &mut HashMap::new(),
        &mut commands,
        &mut Env::default(),
    );

    assert!(result.is_err());
}

#[test]
fn eval_condition_for_slice_empty() {
    let result = eval_condition_for_slice(&vec![]);

    let output = result.unwrap();
    assert!(!output);
}

#[test]
fn eval_condition_for_slice_true() {
    let result = eval_condition_for_slice(&vec!["true".to_string()]);

    let output = result.unwrap();
    assert!(output);
}

#[test]
fn eval_condition_for_slice_false() {
    let result = eval_condition_for_slice(&vec!["false".to_string()]);

    let output = result.unwrap();
    assert!(!output);
}

#[test]
fn eval_condition_for_slice_true_and_false() {
    let result = eval_condition_for_slice(&vec![
        "true".to_string(),
        "and".to_string(),
        "false".to_string(),
    ]);

    let output = result.unwrap();
    assert!(!output);
}

#[test]
fn eval_condition_for_slice_false_and_true() {
    let result = eval_condition_for_slice(&vec![
        "false".to_string(),
        "and".to_string(),
        "true".to_string(),
    ]);

    let output = result.unwrap();
    assert!(!output);
}

#[test]
fn eval_condition_for_slice_true_or_false() {
    let result = eval_condition_for_slice(&vec![
        "true".to_string(),
        "or".to_string(),
        "false".to_string(),
    ]);

    let output = result.unwrap();
    assert!(output);
}

#[test]
fn eval_condition_for_slice_false_or_true() {
    let result = eval_condition_for_slice(&vec![
        "false".to_string(),
        "or".to_string(),
        "true".to_string(),
    ]);

    let output = result.unwrap();
    assert!(output);
}

#[test]
fn eval_condition_for_slice_complex_no_parts() {
    let mut result = eval_condition_for_slice(&vec![
        "false".to_string(),
        "or".to_string(),
        "true".to_string(),
        "or".to_string(),
        "false".to_string(),
    ]);
    assert!(result.unwrap());

    result = eval_condition_for_slice(&vec![
        "true".to_string(),
        "and".to_string(),
        "false".to_string(),
        "or".to_string(),
        "true".to_string(),
        "and".to_string(),
        "false".to_string(),
        "or".to_string(),
        "true".to_string(),
    ]);
    assert!(result.unwrap());

    result = eval_condition_for_slice(&vec![
        "false".to_string(),
        "and".to_string(),
        "false".to_string(),
        "or".to_string(),
        "true".to_string(),
        "and".to_string(),
        "false".to_string(),
        "or".to_string(),
        "true".to_string(),
    ]);
    assert!(!result.unwrap());

    result = eval_condition_for_slice(&vec![
        "true".to_string(),
        "and".to_string(),
        "false".to_string(),
        "or".to_string(),
        "true".to_string(),
        "and".to_string(),
        "false".to_string(),
        "or".to_string(),
        "false".to_string(),
    ]);
    assert!(!result.unwrap());
}

#[test]
fn eval_condition_for_slice_complex_with_parts() {
    let mut result = eval_condition_for_slice(&vec![
        "(".to_string(),
        "false".to_string(),
        "or".to_string(),
        "true".to_string(),
        ")".to_string(),
    ]);
    assert!(result.unwrap());

    result = eval_condition_for_slice(&vec![
        "(".to_string(),
        "false".to_string(),
        "or".to_string(),
        "(".to_string(),
        "true".to_string(),
        ")".to_string(),
        ")".to_string(),
    ]);
    assert!(result.unwrap());

    result = eval_condition_for_slice(&vec![
        "true".to_string(),
        "and".to_string(),
        "(".to_string(),
        "false".to_string(),
        "or".to_string(),
        "true".to_string(),
        ")".to_string(),
        "and".to_string(),
        "false".to_string(),
        "or".to_string(),
        "false".to_string(),
    ]);
    assert!(!result.unwrap());

    result = eval_condition_for_slice(&vec![
        "true".to_string(),
        "and".to_string(),
        "(".to_string(),
        "false".to_string(),
        "or".to_string(),
        "true".to_string(),
        "and".to_string(),
        "false".to_string(),
        ")".to_string(),
        "or".to_string(),
        "true".to_string(),
    ]);
    assert!(result.unwrap());

    result = eval_condition_for_slice(&vec![
        "(".to_string(),
        "(".to_string(),
        "(".to_string(),
        ")".to_string(),
        ")".to_string(),
        ")".to_string(),
    ]);
    assert!(!result.unwrap());

    result = eval_condition_for_slice(&vec![
        "true".to_string(),
        "and".to_string(),
        "false".to_string(),
        "or".to_string(),
        "true".to_string(),
        "and".to_string(),
        "false".to_string(),
        "or".to_string(),
        "(".to_string(),
        "true".to_string(),
        "and".to_string(),
        "true".to_string(),
        "or".to_string(),
        "false".to_string(),
        ")".to_string(),
        "and".to_string(),
        "false".to_string(),
    ]);
    assert!(!result.unwrap());

    result = eval_condition_for_slice(&vec![
        "true".to_string(),
        "and".to_string(),
        "false".to_string(),
        "or".to_string(),
        "true".to_string(),
        "and".to_string(),
        "false".to_string(),
        "or".to_string(),
        "(".to_string(),
        "true".to_string(),
        "and".to_string(),
        "true".to_string(),
        "or".to_string(),
        "false".to_string(),
        ")".to_string(),
    ]);
    assert!(result.unwrap());
}

#[test]
fn eval_condition_for_slice_parse_errors() {
    let mut result = eval_condition_for_slice(&vec!["or".to_string()]);
    assert!(result.is_err());

    result = eval_condition_for_slice(&vec!["and".to_string()]);
    assert!(result.is_err());

    result = eval_condition_for_slice(&vec!["(".to_string()]);
    assert!(result.is_err());

    result = eval_condition_for_slice(&vec![")".to_string()]);
    assert!(result.is_err());

    result = eval_condition_for_slice(&vec!["(".to_string(), "true".to_string()]);
    assert!(result.is_err());

    result = eval_condition_for_slice(&vec![
        "false".to_string(),
        "or".to_string(),
        "true".to_string(),
        ")".to_string(),
    ]);
    assert!(result.is_err());

    result = eval_condition_for_slice(&vec!["false".to_string(), "true".to_string()]);
    assert!(result.is_err());

    result = eval_condition_for_slice(&vec![
        "true".to_string(),
        "or".to_string(),
        "or".to_string(),
        "true".to_string(),
    ]);
    assert!(result.is_err());

    result = eval_condition_for_slice(&vec![
        "true".to_string(),
        "and".to_string(),
        "and".to_string(),
        "true".to_string(),
    ]);
    assert!(result.is_err());

    result = eval_condition_for_slice(&vec![
        "true".to_string(),
        "and".to_string(),
        "or".to_string(),
        "true".to_string(),
    ]);
    assert!(result.is_err());

    result = eval_condition_for_slice(&vec![
        "true".to_string(),
        "or".to_string(),
        "and".to_string(),
        "true".to_string(),
    ]);
    assert!(result.is_err());
}
