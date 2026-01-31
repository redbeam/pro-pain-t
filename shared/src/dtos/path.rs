use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PathDto {
    pub path: String,
}

impl PathDto {
    pub fn new(path: impl ToString) -> Self {
        Self { path: path.to_string() }
    }
}
