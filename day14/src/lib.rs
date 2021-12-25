//! Library module with all the logic

use std::collections::HashMap;

/// Solve Advent of Code day 14 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 14.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
    solve_at_depth(data, 10)
}

fn parse_rules(data: &str) -> HashMap<(char, char), char> {
    data.split("\n\n").nth(1).unwrap()
        .split('\n')
        .map(|x| match x.split(" -> ").collect::<Vec<&str>>().get(..2) {
            Some([a, b]) => {
                let mut c = a.chars();
                ((c.next().unwrap(),
                    c.next().unwrap()),
                    b.chars().next().unwrap())
            },
            _ => panic!("at the cisco")
        })
        .collect::<HashMap<(char, char), char>>()
}

/// Solve Advent of Code day 14 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 14.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
    solve_at_depth(data, 40)
}

/// Compute the solution for a given depth
///
/// # Panics
///
/// May panic if data is not formatted correctly.
#[must_use]
pub fn solve_at_depth(data: &str, depth: usize) -> usize {
    let seq0 = data.split("\n\n").next().unwrap();
    // Parse rules into a dict
    let rules: HashMap<(char, char), char> = parse_rules(data);
    let mut memo: HashMap<(usize, char, char), HashMap<char, usize>> = HashMap::new();
    let mut answers: HashMap<char, usize> = HashMap::new();
    seq0.chars()
        .collect::<Vec<char>>()
        .as_slice()
        .windows(2)
        .for_each(|v| {
            if !memo.contains_key(&(depth, v[0], v[1])) {
                let d = compute_depth(depth, v[0], v[1], &rules, &mut memo);
                memo.insert((depth, v[0], v[1]), d);
            }
            let res = memo.get(&(depth, v[0], v[1])).unwrap();
            for (&key, val) in res {
                *answers.entry(key).or_insert(0) += val;
            }
        });
    // Remove the central characters from answer
    seq0.chars()
        .skip(1)
        .for_each(|v| *answers.entry(v).or_insert(1) -= 1);
    *answers.entry(seq0.chars().last().unwrap()).or_insert(0) += 1;
    answers.values().max().unwrap() - answers.values().min().unwrap()
}

fn compute_depth(r: usize, one: char, two: char, rules: &HashMap<(char, char), char>, mem: &mut HashMap<(usize, char, char), HashMap<char, usize>>) -> HashMap<char, usize> {
    // First off, if we're at depth 0, we stop
    if r == 0 {
        let mut k = HashMap::new();
        k.insert(one, 1);
        *k.entry(two).or_insert(0) += 1;
        k
    } else {
    // Either...
    match mem.get(&(r, one, two)) {
        // You have it memoized
        Some(d) => {
            d.clone()
        },
        // Or you compute it...
        None => {
            // Find any rule that can apply
            if let Some(&c) = rules.get(&(one, two)) {
                // Iterate
                let d1 = compute_depth(r-1, one, c, rules, mem);
                let d2 = compute_depth(r-1, c, two, rules, mem);
                // Remove one on c to not count it twice
                let mut d3 = merge(d1, d2);
                *d3.entry(c).or_insert(1) -= 1;
                mem.insert((r, one, two), d3.clone());
                d3
            } else {
                // If there is no rule, this is the end
                let mut k = HashMap::new();
                k.insert(one, 1);
                *k.entry(two).or_insert(0) += 1;
                k
            }
        }
    }
    }
}

fn merge(a: HashMap<char, usize>, b: HashMap<char, usize>) -> HashMap<char, usize> {
    let mut k = HashMap::new();
    k.extend(a);
    for (key, val) in b {
        *k.entry(key).or_insert(0) += val;
    }
    k
}
