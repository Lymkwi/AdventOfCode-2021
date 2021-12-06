//! Library module with all the logic

use std::collections::HashMap;

/// Solve Advent of Code day 04 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 04.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
    // Split list of numbers and board data
    let great_rift = data.split("\n\n").collect::<Vec<&str>>();
    let numbies = great_rift.get(0).unwrap().split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut boards = great_rift.into_iter().skip(1)
        .map(Board::from_str)
        .collect::<Vec<Board>>();
    for (p, &i) in numbies.iter().enumerate() {
        for b in &mut boards {
            if b.play(i) {
                // Congrats!
                return b.sum_empty(&numbies[..=p]) * i
            }
        }
    }
    0
}

struct Board {
    ysum: [usize; 5],
    xsum: [usize; 5],
    data: HashMap<usize, (usize, usize)>
}

impl Board {
    fn sum_empty(&self, data: &[usize]) -> usize {
        self.data.keys()
            .filter(|x| !data.contains(x))
            .sum()
    }
    fn play(&mut self, n: usize) -> bool {
        self.data.contains_key(&n) &&
            {
                let (x,y) = self.data.get(&n).unwrap();
                self.ysum[*y] += 1;
                self.xsum[*x] += 1;
                self.ysum[*y] == 5 || self.xsum[*x] == 5
            }
    }
    fn from_str(init: &str) -> Board {
        let nums = init.split('\n')
            .enumerate() // gives the y number
            .flat_map(move |(y, d)| d.trim().replace("  ", " ").split(' ')
                 .enumerate()
                 .map(move |(x, s)| (s.parse::<usize>().unwrap(), (y, x)))
                 .collect::<Vec<(usize, (usize, usize))>>()
            )
            .collect::<HashMap<usize, (usize, usize)>>();
        Board {
            ysum: [0, 0, 0, 0, 0],
            xsum: [0, 0, 0, 0, 0],
            data: nums
        }
    }
}

/// Solve Advent of Code day 04 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 04.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
    // Split list of numbers and board data
    let great_rift = data.split("\n\n").collect::<Vec<&str>>();
    let numbies = great_rift.get(0).unwrap().split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut boards = great_rift.into_iter().skip(1)
        .map(Board::from_str)
        .collect::<Vec<Board>>();
    let mut bsize = boards.len();
    for (p, &i) in numbies.iter().enumerate() {
        let mut rems: Vec<usize> = vec![];
        for s in 0..bsize {
            let b: &mut Board = boards.get_mut(s).unwrap();
            if b.play(i) {
                if bsize == 1 {
                    // Congrats!
                    return b.sum_empty(&numbies[..=p]) * i
                }
                rems.push(s);
            }
        }
        rems.sort_unstable();
        rems.iter().rev().for_each(|&x| {
            bsize -= 1;
            boards.remove(x);
        });
    }
    0
}

