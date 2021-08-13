//! Contains [Result], [Error] and implementations

use std::fmt;

/// Crate-focused result, using the [Error] enumeration
pub type Result<T> = std::result::Result<T, Error>;

/// Central error variants, representing all possible errors
#[derive(Debug)]
pub enum Error {
    /// Database error
    Database(sqlx::Error),
    /// Request error
    Request(reqwest::Error),
    /// Invalid database error
    InvalidDatabase(sqlx::Error),
    /// Invalid package (json) schema error
    InvalidPackageSchema,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Database(err) => write!(f, "Lockfile error ({:?})", err),
            Error::Request(err) => write!(f, "Request error ({})", err),
            Error::InvalidDatabase(err) => write!(f, "Failed to connect to lockfile ({:?})", err),
            Error::InvalidPackageSchema => write!(f, "Invalid package (json) schema"),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::Database(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Request(err)
    }
}
