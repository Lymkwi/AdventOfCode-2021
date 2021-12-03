extern crate common;
mod lib;

use lib::{solve_part_one, solve_part_two};
use common::read_data;

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
    fn day03_01_example1() {
        let data = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";
        assert_eq!(198, solve_part_one(&data));
    }

    #[test]
    fn day03_02_example1() {
        let data = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";
        assert_eq!(230, solve_part_two(&data));
    }
}
