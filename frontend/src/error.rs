use core::fmt;

#[derive(Debug)]
pub enum FrontendError {
    RequestError(String),
    DeserializationError(String),
    SerializationError(String),
}

impl From<gloo::net::Error> for FrontendError {
    fn from(value: gloo::net::Error) -> Self {
        FrontendError::RequestError(value.to_string())
    }
}

impl fmt::Display for FrontendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestError(e) => write!(f, "{e}"),
            Self::SerializationError(e) => write!(f, "{e}"),
            Self::DeserializationError(msg) => write!(f, "{msg}"),
        }
    }
}
