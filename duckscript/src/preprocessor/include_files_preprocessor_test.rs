use super::*;

#[test]
fn run_no_arguments() {
    let output = run(&None);

    assert!(output.is_ok());
    assert!(output.unwrap().is_empty());
}

#[test]
fn run_single_file() {
    let output = run(&Some(vec!["./src/test/scripts/simple.ds"]));

    assert!(output.is_ok());
    assert!(output.unwrap().len(), 6);
}