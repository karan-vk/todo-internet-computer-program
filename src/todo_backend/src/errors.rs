use candid::CandidType;
use thiserror::Error;

/// Represents the different types of errors that can occur in the application.
#[derive(CandidType, Debug, Error)]
pub(crate) enum Error {
    /// Error indicating that the requested item was not found.
    #[error("Item not found")]
    NotFound,

    /// Error indicating that the input provided was invalid.
    /// The invalid input is included as a string.
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
