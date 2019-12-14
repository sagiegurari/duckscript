#[derive(Debug, PartialEq)]
pub enum DuckScriptError {
    PreProcessNoCommandFound,
    ControlWithoutValidValue,
    MissingEndQuotes,
}
