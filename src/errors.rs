use std::fmt;

/// In-house result type mapping to given type or similar in-house error enum
pub type Result<T> = std::result::Result<T, Error>;

/// In-house error enum, representing crate-specific errors which may occur
pub enum Error {
    /// Database error
    Database(sqlx::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Database(err) => write!(f, "Database error ({})", err),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::Database(err)
    }
}
