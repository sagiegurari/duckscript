use super::*;
use crate::sdk::std::fs::read_text;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args_provided() {
    test::run_script_and_error(vec![create("")], "out = digest", "out");
}

#[test]
fn run_no_content_provided() {
    test::run_script_and_error(vec![create("")], "out = digest --algo sha256", "out");
}

#[test]
fn run_no_file_provided() {
    test::run_script_and_error(vec![create("")], "out = digest --algo sha256 --file", "out");
}

#[test]
fn run_no_algo_provided() {
    test::run_script_and_error(vec![create("")], "out = digest test", "out");
}

#[test]
fn run_algo_not_supported() {
    test::run_script_and_error(vec![create("")], "out = digest --algo badalgo test", "out");
}

#[test]
fn run_sha256_file() {
    test::run_script_and_validate(
        vec![create("")],
        "out = digest --algo sha256 --file ../LICENSE",
        CommandValidation::Match(
            "out".to_string(),
            "CB5E8E7E5F4A3988E1063C142C60DC2DF75605F4C46515E776E3ACA6DF976E14".to_string(),
        ),
    );
}

#[test]
fn run_sha256_string() {
    test::run_script_and_validate(
        vec![create(""), read_text::create("")],
        r#"
        text = readfile ../LICENSE
        out = digest --algo sha256 ${text}
        "#,
        CommandValidation::Match(
            "out".to_string(),
            "CB5E8E7E5F4A3988E1063C142C60DC2DF75605F4C46515E776E3ACA6DF976E14".to_string(),
        ),
    );
}

#[test]
fn run_sha512_file() {
    test::run_script_and_validate(
        vec![create("")],
        "out = digest --algo sha512 --file ../LICENSE",
        CommandValidation::Match(
            "out".to_string(),
            "46CD9BA0455E2EEDDB70B7C793A6476CFBB75FA306C3E3E4F66973CB3E4F3143A358EE6DD3B065D17BA06B2D63C2BC7CAB8E1D01EDE19A3EAA4FC18CE952CF65".to_string(),
        ),
    );
}

#[test]
fn run_sha512_string() {
    test::run_script_and_validate(
        vec![create(""), read_text::create("")],
        r#"
        text = readfile ../LICENSE
        out = digest --algo sha512 ${text}
        "#,
        CommandValidation::Match(
            "out".to_string(),
            "46CD9BA0455E2EEDDB70B7C793A6476CFBB75FA306C3E3E4F66973CB3E4F3143A358EE6DD3B065D17BA06B2D63C2BC7CAB8E1D01EDE19A3EAA4FC18CE952CF65".to_string(),
        ),
    );
}
