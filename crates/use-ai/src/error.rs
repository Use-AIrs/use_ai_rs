use inquire::InquireError;
use thiserror::Error;
use lib_store::error::StagingError;

pub type Result<T> = core::result::Result<T, ToolError>;

#[derive(Debug, Error)]
pub enum ToolError {
    #[error(transparent)]
    UseAiMenuError(#[from] InquireError),
    #[error(transparent)]
    StagingError(#[from] StagingError),
    #[error("Unknown Error")]
    UseAiError,
}