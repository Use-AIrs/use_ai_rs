use serde::Serialize;
use std::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    CsvPhraseError(String),
    CsvBuildError(String),
    IoError(String),
    DataBuilderError(String),
    ModelIdNotFound(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CsvPhraseError(msg) => write!(f, "CSV Phrase Error: {}", msg),
            Error::CsvBuildError(msg) => write!(f, "CSV Build Error: {}", msg),
            Error::IoError(msg) => write!(f, "IO Error: {}", msg),
            Error::DataBuilderError(msg) => write!(f, "Data Builder Error: {}", msg),
            Error::ModelIdNotFound(msg) => write!(f, "Model ID Not Found: {}", msg),
        }
    }
}

impl std::error::Error for Error {}