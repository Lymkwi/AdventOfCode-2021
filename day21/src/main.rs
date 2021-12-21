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
    use common::test;

    test!(day21_01_example1, 1, 739785, "Player 1 starting position: 4\nPlayer 2 starting position: 8");
    test!(day21_02_example1, 2, 444356092776315, "Player 1 starting position: 4\nPlayer 2 starting position: 8");
}
