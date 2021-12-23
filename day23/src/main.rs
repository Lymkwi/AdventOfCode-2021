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

    test!(day23_01_example1, 1, 12521, "#############\n#...........#\n###B#C#B#D###\n  #A#D#C#A#\n  #########");
    test!(day23_02_example1, 2, 44169, "#############\n#...........#\n###B#C#B#D###\n  #A#D#C#A#\n  #########");
}
