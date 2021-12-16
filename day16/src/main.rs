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

    macro_rules! test {
        ($fn:ident, $part:literal, $exp:literal, $data:literal) => {
            #[test]
            fn $fn() {
                let data = $data;
                let expected = $exp;
                match $part {
                    1 => assert_eq!(expected, solve_part_one(&data)),
                    2 => assert_eq!(expected, solve_part_two(&data)),
                    _ => panic!("Wrong day part '{}'", $part)
                }
            }
        }
    }

    test!(day16_01_example1, 1, 16, "8A004A801A8002F478");
    test!(day16_01_example2, 1, 12, "620080001611562C8802118E34");
    test!(day16_01_example3, 1, 23, "C0015000016115A2E0802F182340");
    test!(day16_01_example4, 1, 31, "A0016C880162017C3686B18A3D4780");

    test!(day16_02_example1, 2, 3, "C200B40A82");
    test!(day16_02_example2, 2, 54, "04005AC33890");
    test!(day16_02_example3, 2, 7, "880086C3E88112");
    test!(day16_02_example4, 2, 9, "CE00C43D881120");
    test!(day16_02_example5, 2, 1, "D8005AC2A8F0");
    test!(day16_02_example6, 2, 0, "F600BC2D8F");
    test!(day16_02_example7, 2, 0, "9C005AC2F8F0");
    test!(day16_02_example8, 2, 1, "9C0141080250320F1802104A08");
}
