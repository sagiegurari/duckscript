use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
#[ignore]
fn run_to_directory_does_not_exist() {
    test::run_script_and_fail(vec![create("")], "out = cd test123_bad");
}

#[test]
#[ignore]
fn run_no_arguments() {
    let current_dir = env::current_dir().unwrap().to_string_lossy().into_owned();
    let home_dir = home::home_dir().unwrap().to_string_lossy().into_owned();
    test::run_script_and_validate(
        vec![create("")],
        "out = cd",
        CommandValidation::Match("out".to_string(), home_dir),
    );

    match env::set_current_dir(Path::new(&current_dir)) {
        _ => (),
    };
}

#[test]
#[ignore]
fn run_specific_directory() {
    let current_dir = env::current_dir().unwrap().to_string_lossy().into_owned();
    test::run_script_and_validate(
        vec![create("")],
        "out = cd ./src",
        CommandValidation::Contains("out".to_string(), "src".to_string()),
    );
    test::run_script_and_validate(vec![create("")], "cd ..", CommandValidation::None);

    assert_eq!(
        current_dir,
        env::current_dir().unwrap().to_string_lossy().into_owned()
    );
}
