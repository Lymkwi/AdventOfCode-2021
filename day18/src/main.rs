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

    test!(day18_01_example1, 1, 143, "[[1,2],[[3,4],5]]");
    test!(day18_01_example2, 1, 1384, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    test!(day18_01_example3, 1, 445, "[1,1]\n[2,2]\n[3,3]\n[4,4]");
    test!(day18_01_example4, 1, 791, "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]");
    test!(day18_01_example5, 1, 1137, "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]");
    test!(day18_01_example6, 1, 3488, "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]\n[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]\n[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]\n[7,[5,[[3,8],[1,4]]]]\n[[2,[2,2]],[8,[8,1]]]\n[2,9]\n[1,[[[9,3],9],[[9,0],[0,7]]]]\n[[[5,[7,4]],7],1]\n[[[[4,2],2],6],[8,7]]");
    test!(day18_01_example7, 1, 4140, "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n[[[5,[2,8]],4],[5,[[9,9],0]]]\n[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n[[[[5,4],[7,7]],8],[[8,3],8]]\n[[9,3],[[9,9],[6,[4,9]]]]\n[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]");
    test!(day18_02_example1, 2, 3993, "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n[[[5,[2,8]],4],[5,[[9,9],0]]]\n[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n[[[[5,4],[7,7]],8],[[8,3],8]]\n[[9,3],[[9,9],[6,[4,9]]]]\n[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]");
}
