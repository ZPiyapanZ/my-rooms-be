use serde::Deserialize;
use thiserror::Error; 

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>, 
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] diesel::result::Error), // Automatically converts diesel errors
    #[error("Bad request")]
    BadRequest(String), // 400
}