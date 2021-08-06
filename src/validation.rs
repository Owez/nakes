//! Contains validation functions

use crate::{errors::ValidationError, Result};

pub const MIN_NAME: usize = 2;
pub const MAX_NAME: usize = 32;
pub const MIN_VERSION: usize = 0;
pub const MAX_VERSION: usize = 16;
pub const MIN_HASH: usize = 16; // TODO: figure actual size
pub const MAX_HASH: usize = 64; // TODO: figure actual size

/// Validates name value
pub fn validate_name(name: &String) -> Result<()> {
    if name.len() <= MIN_NAME {
        Err(ValidationError::NameTooShort.into())
    } else if name.len() > MAX_NAME {
        Err(ValidationError::NameTooLong.into())
    } else {
        Ok(())
    }
}

/// Validates version value
pub fn validate_version(version: &String) -> Result<()> {
    if version.len() <= MIN_VERSION {
        Err(ValidationError::VersionTooShort.into())
    } else if version.len() > MAX_VERSION {
        Err(ValidationError::VersionTooLong.into())
    } else {
        Ok(())
    }
}

/// Validates hash value
pub fn validate_hash(hash: &String) -> Result<()> {
    if hash.len() <= MIN_HASH {
        Err(ValidationError::HashTooShort.into())
    } else if hash.len() > MAX_HASH {
        Err(ValidationError::HashTooLong.into())
    } else {
        Ok(())
    }
}
