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

    const DATA: &'static str = "5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526";

    #[test]
    fn day11_01_example1() {
        let expected = 1656;
        assert_eq!(expected, solve_part_one(DATA));
    }

    #[test]
    fn day11_02_example1() {
        let expected = 195;
        assert_eq!(expected, solve_part_two(DATA));
    }
}
