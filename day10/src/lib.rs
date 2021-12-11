//! Library module with all the logic

use std::collections::VecDeque;

/// Solve Advent of Code day 10 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 10.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
    data.trim().split('\n')
        .map(|x| line_score(x).0)
        .sum::<usize>()
}

#[derive(PartialEq)]
enum Delim {
    None,   // None
    Paren,  // Parenthesis      ()
    Sqbra,  // Square Brackets  []
    Cubra,  // Curly Brackets   {}
    Chevr   // Chevron          <>
}

fn line_score(line: &str) -> (usize, Option<VecDeque<Delim>>) {
    // A mere incomplete line counts for 0, since it's
    let mut stack: VecDeque<Delim> = VecDeque::new();
    for x in line.chars() {
        match x {
            '(' => stack.push_front(Delim::Paren),
            '[' => stack.push_front(Delim::Sqbra),
            '{' => stack.push_front(Delim::Cubra),
            '<' => stack.push_front(Delim::Chevr),
            ')' => if stack.pop_front().unwrap_or(Delim::None) != Delim::Paren { return (3, None); },
            ']' => if stack.pop_front().unwrap_or(Delim::None) != Delim::Sqbra { return (57, None); },
            '}' => if stack.pop_front().unwrap_or(Delim::None) != Delim::Cubra { return (1197, None); },
            '>' => if stack.pop_front().unwrap_or(Delim::None) != Delim::Chevr { return (25137, None); }
             _   => panic!("At the cisco")
        }
    }
    (0, Some(stack))
}

/// Solve Advent of Code day 10 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 10.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
    let mut res: Vec<usize> = data.trim().split('\n')
        .map(|x| line_score(x))
        .filter(|(s, _)| *s == 0) // Incomplete lines only
        .map(|(_, q)| q.unwrap().iter().fold(0, |s, v| s * 5 + match v {
            Delim::None => 0,
            Delim::Paren => 1,
            Delim::Sqbra => 2,
            Delim::Cubra => 3,
            Delim::Chevr => 4
        }))
        .collect::<Vec<usize>>();
    res.sort_unstable();
    let s = res.len();
    *res.get(((s+1)/2+(s-1)/2)/2).unwrap()
}

