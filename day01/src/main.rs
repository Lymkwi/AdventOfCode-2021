//! This crates contains the code necessary to solve Advent of Code day 01,
//! all written in Rust.
extern crate common;
use common::read_data;
mod lib;
use lib::{solve_part_one, solve_part_two};

#[doc(hidden)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = read_data("input")?;
    println!("{}", solve_part_one(&data));
    println!("{}", solve_part_two(&data));
    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day01_01_example1() {
        assert_eq!(7, solve_part_one("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n"));
    }

    #[test]
    fn day01_02_example1() {
        assert_eq!(5, solve_part_two("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n"));
    }
}
