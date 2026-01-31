use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SaveObjectDto {
    pub path: String,
    pub project_serialized: String,
}

impl SaveObjectDto {
    pub fn new(path: String, project_serialized: String) -> Self {
        Self { path, project_serialized }
    }
}
