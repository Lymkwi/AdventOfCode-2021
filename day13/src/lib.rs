//! Library module with all the logic

use std::collections::HashSet;

enum FoldInstruction {
    FoldX(usize),
    FoldY(usize)
}

fn fold_from_line(line: &str) -> FoldInstruction {
    match &line.split('=').collect::<Vec<&str>>()[..2] {
        ["fold along x", x] => FoldInstruction::FoldX(x.parse::<usize>().unwrap()),
        ["fold along y", y] => FoldInstruction::FoldY(y.parse::<usize>().unwrap()),
        _ => panic!("Invalid line '{}'", line)
    }
}

fn apply(y: usize, x: usize, fold: &FoldInstruction) -> (usize, usize) {
    match *fold {
        FoldInstruction::FoldX(sx) => (y, if x < sx { x } else { sx-(x-sx) }),
        FoldInstruction::FoldY(sy) => (if y < sy { y } else { sy-(y-sy) }, x)
    }
}

/// Solve Advent of Code day 13 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 13.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
    // Get the instructions first
    let first_instruction: FoldInstruction = fold_from_line(data.split("\n\n").nth(1).unwrap()
        .split('\n')
        .next().unwrap());
    // And now, build the darn data grid
    data.split("\n\n").next().unwrap().split('\n')
        .map(|coords| {
            let decoded = coords.split(',').map(|c| c.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            apply(*decoded.get(1).unwrap(), *decoded.get(0).unwrap(), &first_instruction)
        })
        .collect::<HashSet<(usize, usize)>>()
        .len()
}

/// Solve Advent of Code day 13 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 13.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> String {
    let more_instructions: Vec<FoldInstruction> = data.split("\n\n")
        .nth(1).unwrap()
        .split('\n')
        .map(|fold| fold_from_line(fold))
        .collect::<Vec<FoldInstruction>>();
    let code: HashSet<(usize, usize)> = data.split("\n\n").next()
        .unwrap().split('\n')
        .map(|coords| {
            let decoded = coords.split(',').map(|c| c.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            more_instructions.iter()
                .fold((*decoded.get(1).unwrap(), *decoded.get(0).unwrap()),
                    |(y, x), i| apply(y, x, i))
        })
        .collect::<HashSet<(usize, usize)>>();
        (0..6)
            .map(|y| (0..39)
                 .map(|x| if code.contains(&(y, x)) { '#' } else { ' ' })
                 .collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
}

