//! Library module with all the logic

use std::collections::HashMap;

/// Solve Advent of Code day 07 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 07.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
    let mut numbies: Vec<usize> = data.split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    numbies.sort_unstable();
    let median = match  numbies.len() % 2 {
        0 => *numbies.get(numbies.len() / 2).unwrap(),
        _ => *numbies.get(((numbies.len()-1)/2 + (numbies.len()+1)/2)/2).unwrap()
    };
    numbies.iter().map(|&x| abs(x, median)).sum::<usize>()
}

fn abs(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}

/// Solve Advent of Code day 07 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 07.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
    let numbies: Vec<usize> = data.split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let minpos = *numbies.iter().min().unwrap();
    let maxpos = *numbies.iter().max().unwrap();
    let mut costs: HashMap<usize, usize> = HashMap::new();
    numbies.iter().for_each(|&x|
        // Iterate over every step and fill in the blank
        (minpos..maxpos).for_each(|y| {
            let n = abs(y, x);
            costs.insert(y, costs.get(&y).unwrap_or(&0) + (n*(n+1))/2);
        })
    );
    // Find the minimum
    *costs.values().min().unwrap_or(&0)
}

