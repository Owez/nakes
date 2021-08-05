//! The simple python packaging tool ✨

#![deny(unsafe_code)]
#![deny(missing_docs)]

mod errors;
mod package;

pub use errors::{Error, Result};
pub use package::Package;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
