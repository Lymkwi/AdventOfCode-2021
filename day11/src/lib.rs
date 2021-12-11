//! Library module with all the logic

use std::collections::HashMap;

/// Solve Advent of Code day 11 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 11.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
    let mut grid: HashMap<(usize, usize), usize> = build_grid(data);
    (0..100).map(|_| step_grid(&mut grid))
        .sum::<usize>()
}

fn step_grid(grid: &mut HashMap<(usize, usize), usize>) -> usize {
    // First, increase all by one
    grid.iter_mut()
        .for_each(|(_, v)| *v += 1);
    // While increase, keep looping
    while grid.values().any(|&x| x > 9 && x < 10000) {
        // Get position of all 10s
        let pos: Vec<(usize, usize)> = grid.iter()
            .filter(|(_, &v)| v > 9 && v < 10000)
            .map(|(&k, _)| k).collect::<Vec<(usize, usize)>>();
        // Increase all of these to 10000
        pos.iter().for_each(|k| *grid.get_mut(&k).unwrap() = 10000);
        // Compute flashes
        pos.iter().for_each(|&(y, x)| {
            // Down neighbor
            if y < 9 { *grid.get_mut(&(y+1, x)).unwrap() += 1; }
            // Up neighbour
            if y > 0 { *grid.get_mut(&(y-1, x)).unwrap() += 1; }
            // Right
            if x < 9 { *grid.get_mut(&(y, x+1)).unwrap() += 1; }
            // Left
            if x > 0 { *grid.get_mut(&(y, x-1)).unwrap() += 1; }
            // Left Up
            if x > 0 && y > 0 { *grid.get_mut(&(y-1, x-1)).unwrap() += 1; }
            // Right Up
            if x < 9 && y > 0 { *grid.get_mut(&(y-1, x+1)).unwrap() += 1; }
            // Down left
            if x > 0 && y < 9 { *grid.get_mut(&(y+1, x-1)).unwrap() += 1; }
            // Down right
            if x < 9 && y < 9 { *grid.get_mut(&(y+1, x+1)).unwrap() += 1; }
        });
    }
    let res = grid.values().filter(|&x| *x >= 10000)
        .count();
    grid.iter_mut()
        .filter(|(_, &mut x)| x >= 10000)
        .for_each(|(_, v)| *v = 0);
    res
}

fn build_grid(data: &str) -> HashMap<(usize, usize), usize> {
    data.trim().split('\n')
        .enumerate()
        .flat_map(|(y, line)| line.chars()
                  .enumerate()
                  .map(move |(x, c)|
                       ((y, x), String::from(c).parse::<usize>().unwrap())
                )
        )
        .collect::<HashMap<(usize, usize), usize>>()
}

/// Solve Advent of Code day 11 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 11.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
    let mut grid = build_grid(data);
    let mut c = 0;
    while grid.values().any(|&x| x != 0) {
        c += 1;
        step_grid(&mut grid);
    }
    c
}

