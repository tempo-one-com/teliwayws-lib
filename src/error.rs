use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize)]
pub enum Error {
   Parsing(String),

   MissingParam(String),

   ApiConnexion(String),
   ApiResult(String),
}

impl From<serde_json::Error> for Error {
   fn from(e: serde_json::Error) -> Self {
       Error::Parsing(e.to_string())
   }
}
