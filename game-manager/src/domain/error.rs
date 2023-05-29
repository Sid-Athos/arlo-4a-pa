use axum::http::StatusCode;
use colored::Colorize;
use crate::database::database_error::DatabaseError;

pub fn internal_error<E>(_: E) -> StatusCode where E: std::error::Error {
    StatusCode::INTERNAL_SERVER_ERROR
}

pub fn database_error_to_response_error(err: DatabaseError) -> String {
    match err {
        DatabaseError::NotFound => "Not found".to_string(),
        DatabaseError::DuplicateKey => "Duplicate key".to_string(),
        DatabaseError::InvalidInput => "Invalid input".to_string(),
        DatabaseError::CannotGetConnectionToDatabase => "Cannot get connection to database".to_string(),
    }
}

pub fn database_error_to_status_code(err: DatabaseError) -> StatusCode {
    match err {
        DatabaseError::NotFound => StatusCode::NOT_FOUND,
        DatabaseError::DuplicateKey => StatusCode::CONFLICT,
        DatabaseError::InvalidInput => StatusCode::BAD_REQUEST,
        DatabaseError::CannotGetConnectionToDatabase => StatusCode::SERVICE_UNAVAILABLE,
    }
}

pub fn status_code_to_string(err: StatusCode) -> String {
    err.to_string()
}

pub trait LogError {
    fn log_error(&self);
}

impl LogError for Result<(), String> {

    fn log_error(&self) {
        if self.is_err() {
            println!("{} : {}", "ERROR".red(), self.clone().err().unwrap());
        }
    }
}