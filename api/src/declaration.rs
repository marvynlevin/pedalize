use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ApiError {
    pub(crate) code: u16,
    pub(crate) message: String
}

impl ApiError {
    pub fn new(code: u16, message: impl ToString) -> Self {
        Self {
            code,
            message: message.to_string()
        }
    }
}