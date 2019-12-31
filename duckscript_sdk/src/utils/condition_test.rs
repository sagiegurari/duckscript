use super::*;

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
