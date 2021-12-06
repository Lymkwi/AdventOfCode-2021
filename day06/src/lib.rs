//! Library module with all the logic

use std::collections::HashMap;

/// Solve Advent of Code day 06 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 06.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
    let numbies: Vec<usize> = data.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    numbies.iter().map(|&x| check_depth(x, 80, &mut cache)).sum()
}

fn check_depth(data: usize, rem: usize,
               cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if rem == 0 { 1 } else {
        match cache.get(&(data, rem)) {
            Some(&v) => v,
            None => {
                let u = match data {
                    0 => check_depth(6, rem-1, cache)
                        + check_depth(8, rem-1, cache),
                    v => check_depth(v-1, rem-1, cache)
                };
                cache.insert((data, rem), u);
                u
            }
        }
    }
}

/// Solve Advent of Code day 06 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 06.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
    let numbies: Vec<usize> = data.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    numbies.iter().map(|&x| check_depth(x, 256, &mut cache)).sum()
}
