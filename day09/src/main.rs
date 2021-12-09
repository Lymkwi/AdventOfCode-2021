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
    fn day09_01_example1() {
        let data = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";
        let expected = 15;
        assert_eq!(expected, solve_part_one(&data));
    }

    #[test]
    fn day09_02_example1() {
        let data = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";
        let expected = 1134;
        assert_eq!(expected, solve_part_two(&data));
    }
}
