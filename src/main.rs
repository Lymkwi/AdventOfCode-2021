extern crate common;
extern crate criterion;
extern crate day01;
extern crate day02;

fn main() {
    println!("Use `cargo bench` or `cargo test`.");
}

#[cfg(test)]
mod test {
    use common::read_data;
    #[test]
    fn day01_one() {
        assert_eq!(Ok(1759),
            day01::solve_part_one(&read_data("day01/input").unwrap()))
    }

    #[test]
    fn day01_two() {
        assert_eq!(Ok(1805),
            day01::solve_part_two(&read_data("day01/input").unwrap()))
    }

    #[test]
    fn day02_one() {
        assert_eq!(Ok(1480518),
            day02::solve_part_one(&read_data("day02/input").unwrap()));
    }

    #[test]
    fn day02_two() {
        assert_eq!(Ok(1282809906),
            day02::solve_part_two(&read_data("day02/input").unwrap()));
    }
}
