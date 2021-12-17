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

#[macro_export]
macro_rules! test {
    ($fn:ident, $part:literal, $exp:literal, $data:literal) => {
        #[test]
        fn $fn() {
            let data = $data;
            // The expected definitions are in separate blocks
            // So that the compiler can infer the right type from
            // The right function without us having to tell it
            match $part {
                1 => {
                    let expected = $exp;
                    assert_eq!(expected, solve_part_one(&data))
                },
                2 => {
                    let expected = $exp;
                    assert_eq!(expected, solve_part_two(&data))
                },
                _ => panic!("Wrong day part '{}'", $part)
            }
        }
    }
}
