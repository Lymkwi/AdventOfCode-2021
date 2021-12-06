//! Library module with all the logic
use std::collections::HashMap;

/// Solve Advent of Code day 05 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 05.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
    let mut grid: HashMap<(usize,usize),usize> = HashMap::new();
    // Parse the data and build the line list
    for line in data.trim().split('\n') {
        let ends = line.split(" -> ").collect::<Vec<&str>>();
        let stop_coords = ends.get(0).unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let sx = *stop_coords.get(0).unwrap();
        let sy = *stop_coords.get(1).unwrap();
        let end_coords = ends.get(1).unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let ex = *end_coords.get(0).unwrap();
        let ey = *end_coords.get(1).unwrap();
        if sx == ex {
            // It's a line in the y direction
            let mn = std::cmp::min(sy, ey);
            let ma = sy + ey - mn;
            (mn..=ma).for_each(|y| *grid.entry((y,sx)).or_insert(0) += 1);
        } else if sy == ey {
            let mn = std::cmp::min(sx, ex);
            let ma = sx + ex - mn;
            (mn..=ma).for_each(|x| *grid.entry((sy,x)).or_insert(0) += 1);
        }
    }
    grid.values().copied().filter(|&x| x >= 2).count()
}

/// Solve Advent of Code day 05 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 05.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
    let mut grid: HashMap<(usize,usize),usize> = HashMap::new();
    // Parse the data and build the line list
    for line in data.trim().split('\n') {
        let ends = line.split(" -> ").collect::<Vec<&str>>();
        let stop_coords = ends.get(0).unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let sx = *stop_coords.get(0).unwrap();
        let sy = *stop_coords.get(1).unwrap();
        let end_coords = ends.get(1).unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let ex = *end_coords.get(0).unwrap();
        let ey = *end_coords.get(1).unwrap();
        if sx == ex {
            // It's a line in the y direction
            let mn = std::cmp::min(sy, ey);
            let ma = sy + ey - mn;
            (mn..=ma).for_each(|y| *grid.entry((y,sx)).or_insert(0) += 1);
        } else if sy == ey {
            let mn = std::cmp::min(sx, ex);
            let ma = sx + ex - mn;
            (mn..=ma).for_each(|x| *grid.entry((sy,x)).or_insert(0) += 1);
        } else if chdif(ex, sx) == chdif(ey ,sy) {
            let min_x = std::cmp::min(ex, sx);
            let min_y = std::cmp::min(ey, sy);
            if (ex > sx && ey > sy) || (sy > ey && sx > ex) {
                (0..=chdif(ex, sx)).for_each(|d| {
                    *grid.entry((min_y+d, min_x+d)).or_insert(0) += 1;
                });
            } else if ex > sx && sy > ey {
                (0..=chdif(ex, sx)).for_each(|d| {
                    *grid.entry((sy-d, sx+d)).or_insert(0) += 1;
                });
            } else {
                (0..=chdif(ex, sx)).for_each(|d| {
                    *grid.entry((sy+d, sx-d)).or_insert(0) += 1;
                });
            }
        }
    }
    grid.values().copied().filter(|&x| x >= 2).count()
}

fn chdif(x: usize, y: usize) -> usize {
    if x >= y { x - y } else { y - x }
}
