//! Custom logging macros for uniform logging

use std::{fmt, process};

/// Logger for normal information
pub fn info(msg: impl fmt::Display) {
    println!("{}..", msg)
}

/// Fatally errors with a given message, returning `!` type
pub fn fatal(msg: impl fmt::Display) -> ! {
    eprintln!("{}!", msg);
    process::exit(1)
}
