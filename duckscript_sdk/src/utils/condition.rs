use crate::utils::eval;
use duckscript::types::command::{CommandResult, Commands};
use duckscript::types::env::Env;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./condition_test.rs"]
mod condition_test;

enum FoundToken {
    None,
    And,
    Or,
    Value,
}

pub(crate) fn is_true(value: Option<String>) -> bool {
    let failed = match value {
        Some(value_str) => {
            let lower_case = value_str.to_lowercase();
            lower_case == "" || lower_case == "0" || lower_case == "false" || lower_case == "no"
        }
        None => true,
    };

    !failed
}

pub(crate) fn eval_condition(
    arguments: Vec<String>,
    instructions: &Vec<Instruction>,
    state: &mut HashMap<String, StateValue>,
    variables: &mut HashMap<String, String>,
    commands: &mut Commands,
    env: &mut Env,
) -> Result<bool, String> {
    if arguments.is_empty() {
        Ok(is_true(None))
    } else {
        let eval_statement = commands.exists(&arguments[0]);

        if eval_statement {
            match eval::eval_with_instructions(
                &arguments,
                instructions,
                state,
                variables,
                commands,
                env,
            ) {
                CommandResult::Continue(value) => {
                    let passed = is_true(value);

                    Ok(passed)
                }
                CommandResult::Error(error) => Err(error.to_string()),
                _ => Err("Invalid condition evaluation result.".to_string()),
            }
        } else {
            eval_condition_for_slice(&arguments[..])
        }
    }
}

pub(crate) fn eval_condition_for_slice(arguments: &[String]) -> Result<bool, String> {
    if arguments.is_empty() {
        Ok(is_true(None))
    } else {
        let mut searching_block_end = false;
        let mut start_block = 0;
        let mut counter = 0;
        let mut index = 0;
        let mut total_evaluated = None;
        let mut partial_evaluated = None;
        let mut found_token = FoundToken::None;
        for argument in arguments {
            if argument == "(" {
                searching_block_end = true;
                if counter == 0 {
                    start_block = index + 1
                }
                counter = counter + 1;
            } else if argument == ")" {
                counter = counter - 1;

                if counter == 0 {
                    searching_block_end = false;

                    match eval_condition_for_slice(&arguments[start_block..index]) {
                        Ok(evaluated) => {
                            start_block = 0;

                            match found_token {
                                FoundToken::None => {
                                    total_evaluated = Some(evaluated);
                                    found_token = FoundToken::Value;
                                }
                                FoundToken::And => {
                                    partial_evaluated = Some(evaluated);
                                    found_token = FoundToken::Value;
                                }
                                FoundToken::Or => {
                                    partial_evaluated =
                                        Some(evaluated || partial_evaluated.unwrap_or(false));
                                    found_token = FoundToken::Value;
                                }
                                FoundToken::Value => {
                                    return Err(
                                        format!("Unexpected value: {}", argument).to_string()
                                    );
                                }
                            }
                        }
                        Err(error) => return Err(error),
                    };
                } else if counter < 0 {
                    return Err("Unexpected ')'".to_string());
                }
            } else if !searching_block_end {
                if argument == "and" {
                    match found_token {
                        FoundToken::Value => {
                            found_token = FoundToken::And;

                            total_evaluated = Some(
                                total_evaluated.unwrap_or(true)
                                    && partial_evaluated.unwrap_or(true),
                            );
                            partial_evaluated = None;

                            if !total_evaluated.unwrap() {
                                return Ok(false);
                            }
                        }
                        _ => return Err("Unexpected 'and'".to_string()),
                    }
                } else if argument == "or" {
                    match found_token {
                        FoundToken::Value => found_token = FoundToken::Or,
                        _ => return Err("Unexpected 'or'".to_string()),
                    }
                } else {
                    let evaluated = is_true(Some(argument.to_string()));

                    match found_token {
                        FoundToken::None => {
                            partial_evaluated = Some(evaluated);
                            found_token = FoundToken::Value;
                        }
                        FoundToken::And => {
                            partial_evaluated = Some(evaluated);
                            found_token = FoundToken::Value;
                        }
                        FoundToken::Or => {
                            partial_evaluated =
                                Some(evaluated || partial_evaluated.unwrap_or(false));
                            found_token = FoundToken::Value;
                        }
                        FoundToken::Value => {
                            return Err(format!("Unexpected value: {}", argument).to_string());
                        }
                    }
                }
            }

            index = index + 1;
        }

        if searching_block_end {
            Err("Missing ')'".to_string())
        } else {
            let total_bool = if total_evaluated.is_none() && partial_evaluated.is_none() {
                is_true(None)
            } else {
                partial_evaluated.unwrap_or(true) && total_evaluated.unwrap_or(true)
            };

            Ok(total_bool)
        }
    }
}
