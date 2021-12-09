//! Library module with all the logic

use std::collections::HashMap;
use std::collections::HashSet;

/// Solve Advent of Code day 09 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 09.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
    let data = data.split('\n')
        .enumerate()
        .flat_map(|(y, d)| d.chars()
             .map(|u| String::from(u).parse::<usize>().unwrap())
             .enumerate()
             .map(|(x, u)| ((y, x), u))
              .collect::<Vec<((usize, usize), usize)>>()
        )
        .collect::<HashMap<(usize, usize), usize>>();
    data.iter()
        .filter(|((y, x), &u)| {
            u < *data.get(&(y+1, *x))
                    .unwrap_or(&usize::MAX)
                && u < *data.get(&(*y, x+1))
                    .unwrap_or(&usize::MAX)
                && (*y == 0 || u < *data.get(&(y-1, *x)).unwrap())
                && (*x == 0 || u < *data.get(&(*y, x-1)).unwrap())
        })
        .map(|(_, &x)| x+1)
        .sum::<usize>()
}

/// Solve Advent of Code day 09 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 09.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
    let data = data.split('\n')
        .enumerate()
        .flat_map(|(y, d)| d.chars()
             .map(|u| String::from(u).parse::<usize>().unwrap())
             .enumerate()
             .map(|(x, u)| ((y, x), u))
              .collect::<Vec<((usize, usize), usize)>>()
        )
        .collect::<HashMap<(usize, usize), usize>>();
    // Find bottoms again
    let mut sizes: Vec<usize> = data.iter()
        .filter(|((y, x), &u)| {
            u < *data.get(&(y+1, *x))
                    .unwrap_or(&usize::MAX)
                && u < *data.get(&(*y, x+1))
                    .unwrap_or(&usize::MAX)
                && (*y == 0 || u < *data.get(&(y-1, *x)).unwrap())
                && (*x == 0 || u < *data.get(&(*y, x-1)).unwrap())
        })
        .map(|((y,x),_)| pool_size(&data, *y, *x))
        .collect();
    sizes.sort_unstable();
    sizes.iter().rev().take(3).product()
 }

fn pool_size(data: &HashMap<(usize, usize), usize>, sy: usize, sx: usize) -> usize {
    let mut inspect: Vec<(usize, usize)> = vec![(sy, sx)];
    let mut done: HashSet<(usize, usize)> = HashSet::new();
    while !inspect.is_empty() {
        let (y, x): (usize, usize) = inspect.pop().unwrap();
        // Add all neighbours that have +1 on us
        let val: usize = *data.get(&(y, x)).unwrap();
        // 9 can't be part of any pool
        if val == 9 { continue; }
        // A pool is a place where it eventually falls down to a single point
        // Check all combinations of two
        let (up, left, down, right) = (
            if y > 0 { *data.get(&(y-1, x)).unwrap() } else { usize::MAX },
            if x > 0 { *data.get(&(y, x-1)).unwrap() } else { usize::MAX },
            *data.get(&(y+1, x)).unwrap_or(&usize::MAX),
            *data.get(&(y, x+1)).unwrap_or(&usize::MAX)
        );
        done.insert((y, x));
        // There is a lot of duplicated code here because
        // Rust won't allow me to add anything to a usize
        // that can be negative for two reasons
        if down > val && down < 9
            && !done.contains(&(y+1, x))
            && !inspect.contains(&(y+1, x)) {
            inspect.push((y+1, x));
        }
        if right > val && right < 9
            && !done.contains(&(y, x+1))
            && !inspect.contains(&(y, x+1)) {
            inspect.push((y, x+1));
        }
        if y > 0 && up > val && up < 9
            && !done.contains(&(y-1, x))
            && !inspect.contains(&(y-1, x)) {
            inspect.push((y-1, x));
        }
        if x > 0 && left > val && left < 9
            && !done.contains(&(y, x-1))
            && !inspect.contains(&(y, x-1)) {
            inspect.push((y, x-1));
        }
    }
    done.len()
}
