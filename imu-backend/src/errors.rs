use actix_http::{body::BoxBody, StatusCode};
use actix_web::{error::ResponseError, HttpResponse};
use anyhow::Error;
use std::fmt::{Display, Formatter, Result};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum ImuServerError {
    #[error("File not found")]
    FileNotFound,
    #[error("Failed to process data")]
    DataProcessing,
    #[error("Invalid input data")]
    InvalidInputData,
    #[error("Token missing")]
    TokenMissing,
}

#[derive(Debug)]
pub struct ServerResponseError(pub(crate) Error);

impl Display for ServerResponseError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{:?}", self.0)
    }
}

impl ResponseError for ServerResponseError {
    fn status_code(&self) -> StatusCode {
        if let Some(package_server_error) = self.0.root_cause().downcast_ref::<ImuServerError>()
        {
            return match package_server_error {
                ImuServerError::FileNotFound => StatusCode::NOT_FOUND,
                ImuServerError::TokenMissing => StatusCode::UNAUTHORIZED,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
        }
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::with_body(self.status_code(), format!("{}", self.0)).map_into_boxed_body()
    }
}

impl From<Error> for ServerResponseError {
    fn from(error: Error) -> ServerResponseError {
        ServerResponseError(error)
    }
}
