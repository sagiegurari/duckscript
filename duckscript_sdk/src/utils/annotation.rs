pub(crate) fn parse(string: &str) -> Option<Vec<String>> {
    let modified_string = string.trim();
    let length = modified_string.len();
    if modified_string.starts_with("<") && modified_string.ends_with(">") && length > 2 {
        let end = length - 1;
        let values: Vec<String> = modified_string[1..end]
            .split(',')
            .map(|item| item.to_string())
            .collect();

        Some(values)
    } else {
        None
    }
}
