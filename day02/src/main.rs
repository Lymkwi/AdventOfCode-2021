//! This crate contains the code necessary to solve Advent of Code day 02,
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
    fn day02_01_example1() {
        assert_eq!(150, solve_part_one("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2"));
    }

    #[test]
    fn day02_02_example1() {
        assert_eq!(900, solve_part_two("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2"));
    }

}
