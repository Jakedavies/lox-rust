
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    RuntimeError,
    BreakError
}

#[derive(Debug, Clone)]
pub struct EvaluationError {
    pub kind: ErrorType,
    pub message: String,
}

impl EvaluationError {
    pub fn runtime_error(message: String) -> Self {
        Self { message, kind: ErrorType::RuntimeError }
    }

    pub fn break_error() -> Self {
        Self { kind: ErrorType::BreakError, message: "Unexpected break statement".to_string() }
    }
}
