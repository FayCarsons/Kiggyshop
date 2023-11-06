use core::fmt;

use common::SerdeError;
use web_sys::wasm_bindgen::JsValue;
use yew::AttrValue;

pub type FEResult<T> = Result<T, FrontendError>;

#[derive(Debug, Clone)]
pub enum ErrorType {
    RequestError,
    DeserializationError,
    SerializationError,
    JsError
}

#[derive(Debug, Clone)]
pub struct FrontendError {
    pub _type: ErrorType,
    pub inner: AttrValue
}

impl From<gloo::net::Error> for FrontendError {
    fn from(value: gloo::net::Error) -> Self {
        FrontendError {
            _type: ErrorType::RequestError,
            inner: value.to_string().into() 
        }
    }
}

impl From<SerdeError> for FrontendError {
    fn from(value: SerdeError) -> Self {
        FrontendError { _type: ErrorType::DeserializationError, inner: value.to_string().into() }
    }
}

impl From<JsValue> for FrontendError {
    fn from(value: JsValue) -> Self {
        FrontendError { _type: ErrorType::JsError, inner: format!("{:?}", value).into() }
    }
}

impl From<ErrorType> for String {
    fn from(value: ErrorType) -> Self {
        match value {
            ErrorType::DeserializationError => "Deserialization Errror: ",
            ErrorType::JsError => "JS Error: ",
            ErrorType::RequestError => "Request Error: ",
            ErrorType::SerializationError => "Serialization Error: "
        }.to_owned()
    }
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for FrontendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self._type, self.inner)
    }
}
