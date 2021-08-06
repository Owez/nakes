use std::fmt;

/// In-house result type mapping to given type or similar in-house error enum
pub type Result<T> = std::result::Result<T, Error>;

/// In-house error enum, representing crate-specific errors which may occur
pub enum Error {
    /// Database error
    Database(sqlx::Error),
    /// Validation error
    Validation(ValidationError),
    /// Duplicate item in database found
    DuplicateDatabaseItem,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Database(err) => write!(f, "Database error ({})", err),
            Error::Validation(err) => write!(f, "Validation error, {}", err),
            Error::DuplicateDatabaseItem => write!(f, "Duplicate item in database found"),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::Database(err) // TODO: match to Error::DuplicateDatabaseItem
    }
}

/// Validation errors for common parts of common data items
pub enum ValidationError {
    NameTooShort,
    NameTooLong,
    VersionTooShort,
    VersionTooLong,
    HashTooShort,
    HashTooLong,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ValidationError::NameTooShort => "name is too short",
                ValidationError::NameTooLong => "name is too long",
                ValidationError::VersionTooShort => "version is too short",
                ValidationError::VersionTooLong => "version is too long",
                ValidationError::HashTooShort => "hash is too short",
                ValidationError::HashTooLong => "hash is too long",
            }
        )
    }
}

impl From<ValidationError> for Error {
    fn from(err: ValidationError) -> Self {
        Self::Validation(err)
    }
}
