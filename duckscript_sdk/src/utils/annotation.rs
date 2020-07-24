pub(crate) fn parse(string: &str) -> Option<Vec<String>> {
    let mut modified_string = string.trim();
    if modified_string.starts_with("<") && modified_string.ends_with(">") {
        modified_string = modified_string
            .strip_prefix("<")
            .unwrap()
            .strip_suffix(">")
            .unwrap();

        let values: Vec<String> = modified_string
            .split(',')
            .map(|item| item.to_string())
            .collect();

        Some(values)
    } else {
        None
    }
}
