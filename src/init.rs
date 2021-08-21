//! Allows creation of a new project with specified name

use std::fs::File;
use std::io::prelude::*;

use crate::{log, Error, Result};

/// Creates a nrw lockfiles from bytes embedded into the binary
fn create_lockfile() -> Result<()> {
    log::info("Generating lockfile");

    let lockfile_bytes = include_bytes!("../nakes.lock");

    let mut file = File::create("nakes.lock").map_err(|err| Error::LockfileCreation(err))?;
    file.write_all(lockfile_bytes)
        .map_err(|err| Error::LockfileCreation(err))
}

/// Inits a new lockfile
pub fn run() -> Result<()> {
    create_lockfile()
}
