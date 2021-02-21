use super::*;
use crate::sdk::std::math::calc;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = hex_decode", "out");
}

#[test]
fn run_valid() {
    test::run_script_and_validate(
        vec![create(""), calc::create("")],
        r#"
num = hex_decode 0xFF
out = calc ${num} + 1
"#,
        CommandValidation::Match("out".to_string(), "256".to_string()),
    );
}
