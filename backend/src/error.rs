use core::fmt;
use std::env::VarError;

use actix_web::{HttpResponse, ResponseError};

use serde::{Deserialize, Serialize};

pub type ShopResult<T> = Result<T, BackendError>;

#[derive(Clone, PartialEq, Hash, Serialize, Deserialize, Debug)]
pub enum BackendError {
    ContentNotFound(String),
    EnvError(String),
    FileNotFound(String),
    FileReadError(String),
    FileWriteError(String),
    ResourceLocked(String),
    SerializationError(String),
    DeserializationError(String),
    PaymentError(String),
    Unauthorized,
}

impl From<VarError> for BackendError {
    fn from(value: VarError) -> Self {
        Self::EnvError(value.to_string())
    }
}

impl From<BackendError> for std::io::Error {
    fn from(value: BackendError) -> Self {
        let s = String::from(value);
        Self::other(s)
    }
}

impl From<BackendError> for String {
    fn from(value: BackendError) -> Self {
        match value {
            BackendError::ContentNotFound(s) => format!("Page call {s} not found"),
            BackendError::FileNotFound(s) => format!("API does not contain {s}"),
            BackendError::FileReadError(s) => format!("Cannot read file {s}"),
            BackendError::FileWriteError(s) => format!("Cannot write file {s}"),
            BackendError::ResourceLocked(s) => format!("Resource {s} currently locked"),
            BackendError::SerializationError(s) => format!("Cannot serialize struct {s}"),
            BackendError::DeserializationError(s) => {
                format!("Cannot deserialize file or http response {s}")
            }
            BackendError::Unauthorized => {
                "YOU are not authorized to access this content . . .".to_owned()
            }
            BackendError::EnvError(s) => format!("Environment error: {s}"),
            BackendError::PaymentError(s) => format!("Payment error: {s}"),
        }
    }
}

impl ResponseError for BackendError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            Self::ContentNotFound(s) | Self::FileNotFound(s) => {
                HttpResponse::NotFound().body(s.clone())
            }
            Self::SerializationError(s)
            | Self::DeserializationError(s)
            | Self::FileWriteError(s)
            | Self::EnvError(s)
            | Self::PaymentError(s) => HttpResponse::InternalServerError().body(s.clone()),
            Self::FileReadError(s) => HttpResponse::FailedDependency().body(s.clone()),
            Self::ResourceLocked(s) => HttpResponse::Locked().body(s.clone()),
            Self::Unauthorized => HttpResponse::Forbidden().body(""),
        }
    }
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}
