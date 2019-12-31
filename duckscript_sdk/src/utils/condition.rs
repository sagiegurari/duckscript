#[cfg(test)]
#[path = "./condition_test.rs"]
mod condition_test;

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
