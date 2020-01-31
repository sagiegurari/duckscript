use super::*;
use crate::sdk::std::string::bytes_to_string;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_file_provided() {
    test::run_script_and_error(vec![create("")], "out = readbinfile", "out");
}

#[test]
fn run_file_not_exists() {
    test::run_script_and_error(vec![create("")], "out = readbinfile ./Cargo2.toml", "out");
}

#[test]
fn run_valid() {
    test::run_script_and_validate(
        vec![create(""), bytes_to_string::create("")],
        r#"
handle = readbinfile ./Cargo.toml
out = bytes_to_string ${handle}
"#,
        CommandValidation::Contains("out".to_string(), "duckscript".to_string()),
    );
}
