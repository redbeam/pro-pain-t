use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ProjectDto {
    pub data: Vec<u8>,
}

impl ProjectDto {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}
