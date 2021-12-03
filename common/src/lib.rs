//! Library crate containing the common methods used by multiple days of
//! `AdventOfCode`

use std::fs::File;
use std::io::prelude::*;

/// Read the day's input data from a file.
///
/// Returns a [Result<String>](std::io::Result).
///
/// # Arguments
///
///  - `filepath` : a `&str` holding a reference to the string of the file path
///
/// # Errors
///
/// In case of I/O exception, returns an Error.
pub fn read_data(filepath: &str) -> std::io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string().replace("\r", ""))
}
