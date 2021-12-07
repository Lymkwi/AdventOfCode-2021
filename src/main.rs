extern crate common;
extern crate criterion;
extern crate day01;
extern crate day02;
extern crate day03;
extern crate day04;
extern crate day05;

fn main() {
    println!("Use `cargo bench` or `cargo test`.");
}

#[cfg(test)]
mod test {
    use common::read_data;
    #[test]
    fn day01_one() {
        assert_eq!(1759,
            day01::solve_part_one(&read_data("day01/input").unwrap()))
    }

    #[test]
    fn day01_two() {
        assert_eq!(1805,
            day01::solve_part_two(&read_data("day01/input").unwrap()))
    }

    #[test]
    fn day02_one() {
        assert_eq!(1480518,
            day02::solve_part_one(&read_data("day02/input").unwrap()));
    }

    #[test]
    fn day02_two() {
        assert_eq!(1282809906,
            day02::solve_part_two(&read_data("day02/input").unwrap()));
    }

    #[test]
    fn day03_one() {
        assert_eq!(3958484,
                   day03::solve_part_one(&read_data("day03/input").unwrap()));
    }

    #[test]
    fn day03_two() {
        assert_eq!(1613181,
                   day03::solve_part_two(&read_data("day03/input").unwrap()));
    }

    #[test]
    fn day04_one() {
        assert_eq!(27027,
                   day04::solve_part_one(&read_data("day04/input").unwrap()));
    }

    #[test]
    fn day04_two() {
        assert_eq!(36975,
                   day04::solve_part_two(&read_data("day04/input").unwrap()));
    }

    #[test]
    fn day05_one() {
        assert_eq!(7269,
                   day05::solve_part_one(&read_data("day05/input").unwrap()));
    }

    #[test]
    fn day05_two() {
        assert_eq!(21140,
                   day05::solve_part_two(&read_data("day05/input").unwrap()));
    }

    #[test]
    fn day06_one() {
        assert_eq!(379114,
                   day06::solve_part_one(&read_data("day06/input").unwrap()));
    }

    #[test]
    fn day06_two() {
        assert_eq!(1702631502303,
                   day06::solve_part_two(&read_data("day06/input").unwrap()));
    }

    #[test]
    fn day07_one() {
        assert_eq!(352331,
                   day07::solve_part_one(&read_data("day07/input").unwrap()));
    }

    #[test]
    fn day07_two() {
        assert_eq!(99266250,
                   day07::solve_part_two(&read_data("day07/input").unwrap()));
    }
}
