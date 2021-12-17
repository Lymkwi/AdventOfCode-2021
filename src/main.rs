extern crate criterion;

fn main() {
    println!("Use `cargo bench` or `cargo test`.");
}


#[cfg(test)]
mod test {
    macro_rules! result_tests {
        ($daycrate:ident, $func_one:ident, $func_two:ident, $day:literal, $res1:literal, $res2:literal) => {
            #[test]
            fn $func_one() {
                assert_eq!($res1,
                    $daycrate::solve_part_one(
                        &read_data(&format!("day{:02}/input", $day)).unwrap()))
            }

            #[test]
            fn $func_two() {
                assert_eq!($res2,
                   $daycrate::solve_part_two(
                        &read_data(&format!("day{:02}/input", $day)).unwrap()))
            }
        }
    }

    use common::read_data;

    result_tests!(day01, day01_one, day01_two, 01, 1759, 1805);
    result_tests!(day02, day02_one, day02_two, 02, 1480518, 1282809906);
    result_tests!(day03, day03_one, day03_two, 03, 3958484, 1613181);
    result_tests!(day04, day04_one, day04_two, 04, 27027, 36975);
    result_tests!(day05, day05_one, day05_two, 05, 7269, 21140);
    result_tests!(day06, day06_one, day06_two, 06, 379114, 1702631502303);
    result_tests!(day07, day07_one, day07_two, 07, 352331, 99266250);
    result_tests!(day08, day08_one, day08_two, 08, 416, 1043697);
    result_tests!(day09, day09_one, day09_two, 09, 572, 847044);
    result_tests!(day10, day10_one, day10_two, 10, 215229, 1105996483);
    result_tests!(day11, day11_one, day11_two, 11, 1585, 382);
    result_tests!(day12, day12_one, day12_two, 12, 4720, 147848);
    result_tests!(day13, day13_one, day13_two, 13, 747, " ##  ###  #  # #### ###   ##  #  # #  #\n#  # #  # #  #    # #  # #  # #  # #  #\n#  # #  # ####   #  #  # #    #  # ####\n#### ###  #  #  #   ###  #    #  # #  #\n#  # # #  #  # #    #    #  # #  # #  #\n#  # #  # #  # #### #     ##   ##  #  #");
    result_tests!(day14, day14_one, day14_two, 14, 2587, 3318837563123);
    result_tests!(day15, day15_one, day15_two, 15, 462, 2846);
    result_tests!(day16, day16_one, day16_two, 16, 1002, 1673210814091);
    result_tests!(day17, day17_one, day17_two, 17, 5151, 968);
}
