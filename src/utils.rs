use serde::Serialize;
use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use derive_more::{Display, Error};

#[derive(Serialize)]
struct FormattedErrorResponse {
    status_code: u16,
    error: String,
    message: String,
}

#[derive(Debug, Display, Error)]
pub enum CustomError {
    // Formatting the validation error message
    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },
    // Formatting the internatal server error error message
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
    // Formatting the bad request error message
    #[display(fmt = "Bad request: {}", field)]
    BadClientData {field: String},
    #[display(fmt = "Not found!")]
    NotFound,
}

impl CustomError {
    fn name(&self) -> String {
        match self {
            CustomError::ValidationError { .. } => "Validation Error".to_string(),
            CustomError::InternalError => "Internal Server Error".to_string(),
            CustomError::BadClientData {..} => "Bad request".to_string(),
            CustomError::NotFound => "Not found".to_string(),
        }

    }
}

// Implement ResponseError trait for the custom struct
impl ResponseError for CustomError {
    // Function to generate the error response
    fn error_response(&self) -> HttpResponse {
        let error_response = FormattedErrorResponse {
            status_code: self.status_code().as_u16(),
            error: self.to_string(),
            message: self.name(),
        };
        HttpResponse::build(self.status_code()).json(error_response)
    }
    // Function to generate the error code
    fn status_code(&self) -> StatusCode {
        match *self {
            CustomError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            CustomError::BadClientData {..} => StatusCode::BAD_REQUEST,
            CustomError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}
