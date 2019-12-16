#[derive(Debug, PartialEq)]
pub enum ScriptError {
    PreProcessNoCommandFound,
    ControlWithoutValidValue,
    InvalidControlLocation,
    MissingEndQuotes,
    MissingOutputVariableName,
    InvalidEqualsLocation,
    InvalidQuotesLocation,
    EmptyLabel,
}
