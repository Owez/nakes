//! Contains [Result], [Error] and implementations

use std::fmt;

/// Crate-focused result, using the [Error] enumeration
pub type Result<T> = std::result::Result<T, Error>;

/// Central error variants, representing all possible errors
#[derive(Debug)]
pub enum Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
