//! This crates contains the code necessary to solve Advent of Code day 25,
//! all written in Rust.
//! 
//! Today's puzzle is about the Diffie-Hellman key exchange algorithm.

use std::fs::File;
use std::io::prelude::*;

/// Read the day's input data from a file.
///
/// Returns a [Result<String>](std::io::Result).
///
/// # Arguments
///
///  - `filepath` : a `&str` holding a reference to the string of the file path
fn read_data(filepath: &str) -> std::io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string().replace("\r", ""))
}

/// Solve Advent of Code day 01 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input formatted
///  as such : `<depth integer>\n` an unknown amount of times.
///
/// # Return value
///
/// This function returns a `Result<usize,()>` where `Ok` holds the result for part
/// one of advent of code day 01.
///
/// # Errors
///
/// There is no custom error type here so `Err` always contains `()`.
fn solve_part_one(data: &str) -> Result<usize,()> {
    // Build the data into a Vec<i32>
    Ok(data.trim().split('\n')
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .as_slice()
        .windows(2)
        .fold(0, |c, i| if i[0] < i[1] { c + 1 } else { c }))
}

/// Solve Advent of Code day 01 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input formatted
///  as such : `<depth integer>\n` an unknown amount of times.
///
/// # Return value
///
/// This function returns a `Result<usize,()>` where `Ok` holds the result for part
/// two of advent of code day 01.
///
/// # Errors
///
/// There is no custom error type here so `Err` always contains `()`.
fn solve_part_two(data: &str) -> Result<usize,()> {
    Ok(data.trim().split('\n')
       .map(|v| v.parse::<i32>().unwrap())
       .collect::<Vec<i32>>()
       .as_slice()
       .windows(3)
       .map(|x| x[0] + x[1] + x[2])
       .collect::<Vec<i32>>()
       .as_slice()
       .windows(2)
       .fold(0, |c, i| if i[0] < i[1] { c + 1 } else { c }))
}

#[doc(hidden)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = read_data("input")?;
    println!("{:?}", solve_part_one(&data));
    println!("{:?}", solve_part_two(&data));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day01_01_example1() {
        assert_eq!(Ok(7), solve_part_one("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n"));
    }

    #[test]
    fn day_01_02_example1() {
        assert_eq!(Ok(5), solve_part_two("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n"));
    }
}
