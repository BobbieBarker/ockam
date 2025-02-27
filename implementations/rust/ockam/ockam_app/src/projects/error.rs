use serde::Serialize;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error, Serialize)]
pub enum Error {
    #[error("creating enrollment ticket failed")]
    EnrollmentTicketFailed,
    #[error("decoding enrollment ticket failed")]
    EnrollmentTicketDecodeFailed,
    #[error("listing projects failed: {0:?}")]
    ListingFailed(ockam::Error),
    #[error("binary for ockam command is invalid")]
    OckamCommandInvalid,
    #[error("project {0} not found")]
    ProjectNotFound(String),
    #[error("failed to save local project state")]
    StateSaveFailed,
}

impl From<Error> for String {
    fn from(e: Error) -> Self {
        e.to_string()
    }
}
