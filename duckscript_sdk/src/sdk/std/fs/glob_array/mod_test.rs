use super::*;
use crate::test;
use crate::test::CommandValidation;
use crate::utils::state::get_handles_sub_state;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_pattern_provided() {
    test::run_script_and_error(vec![create("")], "out = glob_array", "out");
}

#[test]
fn run_results_found() {
    let mut result = fsio::directory::create("./target/_duckscript/glob/1/dir1/dir12");
    assert!(result.is_ok());
    result = fsio::directory::create("./target/_duckscript/glob/1/dir2");
    assert!(result.is_ok());
    result = fsio::directory::create("./target/_duckscript/glob/1/dir3/file.txt");
    assert!(result.is_ok());

    let mut context = test::run_script_and_validate(
        vec![create("")],
        r#"out = glob_array ./target/_duckscript/glob/1/**/*"#,
        CommandValidation::Contains("out".to_string(), "handle:".to_string()),
    );

    let state = get_handles_sub_state(&mut context.state);
    let list_value = state.remove(context.variables.get("out").unwrap()).unwrap();
    match list_value {
        StateValue::List(list) => {
            assert_eq!(list.len(), 5);

            let mut paths = vec![];

            for entry in list {
                match entry {
                    StateValue::String(value) => paths.push(value.replace("\\", "/")),
                    _ => panic!("Invalid handle value."),
                };
            }

            paths.sort();

            assert_eq!(
                paths,
                vec![
                    "target/_duckscript/glob/1/dir1",
                    "target/_duckscript/glob/1/dir1/dir12",
                    "target/_duckscript/glob/1/dir2",
                    "target/_duckscript/glob/1/dir3",
                    "target/_duckscript/glob/1/dir3/file.txt"
                ]
            );
        }
        _ => panic!("Invalid handle type."),
    }
}

#[test]
fn run_results_partial_found() {
    let mut result = fsio::directory::create("./target/_duckscript/glob/2/dir1/dir12");
    assert!(result.is_ok());
    result = fsio::directory::create("./target/_duckscript/glob/2/dir2");
    assert!(result.is_ok());
    result = fsio::directory::create("./target/_duckscript/glob/2/dir3/file.txt");
    assert!(result.is_ok());

    let mut context = test::run_script_and_validate(
        vec![create("")],
        r#"out = glob_array ./target/_duckscript/glob/2/**/*.txt"#,
        CommandValidation::Contains("out".to_string(), "handle:".to_string()),
    );

    let state = get_handles_sub_state(&mut context.state);
    let list_value = state.remove(context.variables.get("out").unwrap()).unwrap();
    match list_value {
        StateValue::List(list) => {
            assert_eq!(list.len(), 1);

            let mut paths = vec![];

            for entry in list {
                match entry {
                    StateValue::String(value) => paths.push(value.replace("\\", "/")),
                    _ => panic!("Invalid handle value."),
                };
            }

            paths.sort();

            assert_eq!(paths, vec!["target/_duckscript/glob/2/dir3/file.txt"]);
        }
        _ => panic!("Invalid handle type."),
    }
}
