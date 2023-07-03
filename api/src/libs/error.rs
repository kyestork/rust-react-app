use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppError {
    pub code: usize,
    pub message: String
}