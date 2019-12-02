use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Error {
    error_info: String,
}

impl Error {
    pub fn new(error_info: &str) -> Self {
        Error {
            error_info: error_info.to_string(),
        }
    }
}
