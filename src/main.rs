//! The simple python packaging tool ✨

#![deny(unsafe_code)]
#![deny(missing_docs)]

pub(crate) mod validation;

mod errors;
mod package;

pub use errors::{Error, Result};
pub use package::Package;

use clap::Clap;

/// the simple python packaging tool ✨
#[derive(Debug, Clap)]
enum Commands {
    /// Installs given package name
    Install,
    /// Removes given package name
    Uninstall,
}

#[tokio::main]
async fn main() {
    match Commands::parse() {
        Commands::Install => todo!("install"),
        Commands::Uninstall => todo!("uninstall"),
    }
}
