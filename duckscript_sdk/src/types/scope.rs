use std::collections::HashMap;

pub(crate) fn clear(name: &str, variables: &mut HashMap<String, String>) {
    let mut scope_name = name.to_string();
    scope_name.push_str("::");

    variables.retain(|key, _| !key.starts_with(&scope_name));
}
