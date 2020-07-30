use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = json_parse", "out");
}

#[test]
fn run_all_types() {
    let context = test::run_script_and_validate(
        vec![create("")],
        r#"
out = json_parse "{\"name\": \"my package\", \"version\": 1, \"publish\": false, \"keywords\": [\"test1\", \"test2\"], \"directories\": {\"test\": \"spec\"}}"
"#,
        CommandValidation::Ignore,
    );

    let variables = context.variables;

    assert_eq!(variables.get("out").unwrap(), "[OBJECT]");
    assert_eq!(variables.get("out.name").unwrap(), "my package");
    assert_eq!(variables.get("out.version").unwrap(), "1");
    assert_eq!(variables.get("out.publish").unwrap(), "false");
    assert_eq!(variables.get("out.keywords.length").unwrap(), "2");
    assert_eq!(variables.get("out.keywords[0]").unwrap(), "test1");
    assert_eq!(variables.get("out.keywords[1]").unwrap(), "test2");
    assert_eq!(variables.get("out.directories.test").unwrap(), "spec");
}
