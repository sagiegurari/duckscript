use super::*;

#[test]
fn is_unix_flags_argument_empty() {
    let output = is_unix_flags_argument("");

    assert!(!output);
}

#[test]
fn is_unix_flags_argument_no_flag_prefix() {
    let output = is_unix_flags_argument("abc");

    assert!(!output);
}

#[test]
fn is_unix_flags_argument_double_flag_prefix() {
    let output = is_unix_flags_argument("--abc");

    assert!(!output);
}

#[test]
fn is_unix_flags_argument_valid() {
    let output = is_unix_flags_argument("-abc");

    assert!(output);
}

#[test]
fn is_unix_flag_exists_empty() {
    let output = is_unix_flag_exists('a', "");

    assert!(!output);
}

#[test]
fn is_unix_flag_exists_not_flags() {
    let output = is_unix_flag_exists('a', "abc");

    assert!(!output);
}

#[test]
fn is_unix_flag_exists_not_found() {
    let output = is_unix_flag_exists('r', "abc");

    assert!(!output);
}

#[test]
fn is_unix_flag_exists_found() {
    let output = is_unix_flag_exists('r', "arbc");

    assert!(!output);
}

#[test]
fn is_unix_flag_exists_found_different_case() {
    let output = is_unix_flag_exists('R', "arbc");

    assert!(!output);
}
