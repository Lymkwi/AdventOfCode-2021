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
    fn day07_01_example1() {
        let data = "16,1,2,0,4,2,7,1,2,14";
        let expected = 37;
        assert_eq!(expected, solve_part_one(&data));
    }

    #[test]
    fn day07_02_example1() {
        let data = "16,1,2,0,4,2,7,1,2,14";
        let expected = 168;
        assert_eq!(expected, solve_part_two(&data));
    }
}
