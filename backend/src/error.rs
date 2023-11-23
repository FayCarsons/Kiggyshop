use core::fmt;
use std::{env::VarError, num::ParseIntError};

use actix_web::{error::BlockingError, HttpResponse, ResponseError};

use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;
use stripe::StripeError;

pub type ShopResult<T> = Result<T, BackendError>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum BackendError {
    ContentNotFound(String),
    EnvError(String),
    DbError(String),
    FileNotFound(String),
    FileReadError(String),
    FileWriteError(String),
    ResourceLocked(String),
    SerializationError(String),
    DeserializationError(String),
    PaymentError(String),
    Unauthorized,
    RateLimitError,
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

impl From<std::io::Error> for BackendError {
    fn from(value: std::io::Error) -> Self {
        Self::FileReadError(value.to_string())
    }
}

impl From<diesel::result::Error> for BackendError {
    fn from(value: diesel::result::Error) -> Self {
        Self::DbError(value.to_string())
    }
}

impl From<r2d2::Error> for BackendError {
    fn from(value: r2d2::Error) -> Self {
        Self::DbError(value.to_string())
    }
}

impl From<BlockingError> for BackendError {
    fn from(value: BlockingError) -> Self {
        Self::ResourceLocked(value.to_string())
    }
}

impl From<SerdeError> for BackendError {
    fn from(value: SerdeError) -> Self {
        BackendError::SerializationError(value.to_string())
    }
}

impl From<StripeError> for BackendError {
    fn from(value: StripeError) -> Self {
        Self::PaymentError(value.to_string())
    }
}

impl From<ParseIntError> for BackendError {
    fn from(value: ParseIntError) -> Self {
        Self::DeserializationError(value.to_string())
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
            BackendError::DbError(s) => format!("Error in database: {s}"),
            BackendError::Unauthorized => {
                "YOU are not authorized to access this content . . .".to_owned()
            }
            BackendError::RateLimitError => {
                "Unable to initialize rate limiter middleware".to_owned()
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
            | Self::PaymentError(s)
            | Self::DbError(s) => HttpResponse::InternalServerError().body(s.clone()),
            Self::FileReadError(s) => HttpResponse::FailedDependency().body(s.clone()),
            Self::ResourceLocked(s) => HttpResponse::Locked().body(s.clone()),
            Self::Unauthorized => HttpResponse::Forbidden().body(""),
            Self::RateLimitError => HttpResponse::Ok().finish(),
        }
    }
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}
