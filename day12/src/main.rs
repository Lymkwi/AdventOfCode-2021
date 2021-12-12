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
    fn day12_01_example1() {
        let data = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";
        let expected = 10;
        assert_eq!(expected, solve_part_one(&data));
    }

    #[test]
    fn day12_01_example2() {
        let data = "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc\n";
        let expected = 19;
        assert_eq!(expected, solve_part_one(&data));
    }

    #[test]
    fn day12_01_example3() {
        let data = "fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW";
        let expected = 226;
        assert_eq!(expected, solve_part_one(&data));
    }

    #[test]
    fn day12_02_example1() {
        let data = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";
        let expected = 36;
        assert_eq!(expected, solve_part_two(&data));
    }

    #[test]
    fn day12_02_example2() {
        let data = "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc\n";
        let expected = 103;
        assert_eq!(expected, solve_part_two(&data));
    }

    #[test]
    fn day12_02_example3() {
        let data = "fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW";
        let expected = 3509;
        assert_eq!(expected, solve_part_two(&data));
    }
}
