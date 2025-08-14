use std::fmt::{Display, Formatter};

use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize)]
pub enum Error {
    Parsing(String),

    MissingParam(String),

    ApiConnexion(String),
    ApiResult(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::Parsing(msg) => msg,
            Error::MissingParam(msg) => msg,
            Error::ApiConnexion(msg) => msg,
            Error::ApiResult(msg) => msg,
        };

        write!(f, "{msg}")
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Parsing(e.to_string())
    }
}
