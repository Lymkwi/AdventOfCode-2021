//! Library module with all the logic
mod modification;
mod cuboidunion;
mod utils;
use modification::Modification;
use cuboidunion::CuboidUnion;

/// Solve Advent of Code day 22 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 22.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
    let rules: Vec<Modification> = data.trim().split('\n')
        .map(|x| Modification::from_str(x))
        .rev()
        .collect::<Vec<Modification>>();
    let mut res = 0;
    for x in -50..=50 {
        for y in -50..=50 {
            for z in -50..=50 {
                for r in &rules {
                    if r.is_within((x, y, z)) {
                        res += if r.is_on() { 1 } else { 0 };
                        break;
                    }
                }
            }
        }
    }
    res
}

/// Solve Advent of Code day 22 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 22.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
    // Assemble the rules into an array
    // They have to be in reverse order so that once a rule covers
    // An area it is considered to be defined by that rule
    let rules: Vec<Modification> = data.trim().split('\n')
        .map(|x| Modification::from_str(x))
        .rev()
        .collect::<Vec<Modification>>();
    // This CuboidUnion will account for all of the cubes covered
    // By any of the previous rules. Adding to it will yield the
    // Difference in volume that is caused by the addition of a
    // New cuboid.
    let mut known: CuboidUnion = CuboidUnion::new();
    let mut solution: usize = 0;
    for r in rules {
        let excl_volume = known.add(r.get_cube());
        // If the difference of volume exists on a "on" rule it means
        // We found cubes that were turned on by that rule and never
        // Touched afterwards.
        // Therefore, they are cubes that are turned on.
        if r.is_on() {
            solution += excl_volume;
        }
    }
    solution
}

