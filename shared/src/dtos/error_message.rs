use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub message: String,
}

impl ErrorMessage {
    pub fn new(message: impl ToString) -> Self {
        Self { message: message.to_string() }
    }
}
