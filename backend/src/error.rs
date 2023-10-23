use core::fmt;

use actix_web::{
    web::{Buf, Bytes},
    HttpResponse, ResponseError,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Hash, Serialize, Deserialize, Debug)]
pub enum BackendError {
    ContentNotFound(String),
    FileNotFound(String),
    FileReadError(String),
    FileWriteError(String),
    ResourceLocked(String),
    SerializationError(String),
    DeserializationError(String),
}

impl ResponseError for BackendError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            BackendError::ContentNotFound(s) | BackendError::FileNotFound(s) => {
                HttpResponse::NotFound().body(s.clone())
            }
            BackendError::SerializationError(s)
            | BackendError::DeserializationError(s)
            | BackendError::FileWriteError(s) => {
                HttpResponse::InternalServerError().body(s.clone())
            }
            BackendError::FileReadError(s) => HttpResponse::FailedDependency().body(s.clone()),
            BackendError::ResourceLocked(s) => HttpResponse::Locked().body(s.clone()),
        }
    }
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ContentNotFound(s) => write!(f, "Page call {s} not found"),
            Self::FileNotFound(s) => write!(f, "API does not contain {s}"),
            Self::FileReadError(s) => write!(f, "Cannot read file {s}"),
            Self::FileWriteError(s) => write!(f, "Cannot write file {s}"),
            Self::ResourceLocked(s) => write!(f, "Resource {s} currently locked"),
            Self::SerializationError(s) => write!(f, "Cannot serialize struct {s}"),
            Self::DeserializationError(s) => write!(f, "Cannot deserialize file {s}"),
        }
    }
}
