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

// No tests today :shrug:
