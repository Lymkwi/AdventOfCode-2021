//! Library module with all the logic

use std::collections::HashSet;
use std::collections::HashMap;

/// Solve Advent of Code day 08 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 08.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
    // Finding unique digits just means finding the ones that have 2, 4, 3, or 7 segments
    data.split('\n')
        .map(|x| x.split(" | ")
             .nth(1).unwrap()
             .split(' ')
             .filter(|&x|
                     x.len() == 2 || x.len() == 4 ||
                     x.len() == 3 || x.len() == 7)
             .count()
        ).sum::<usize>()
}

/// Solve Advent of Code day 08 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 08.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
    data.split('\n')
        .map(|x| determine_display_value(x))
        .sum::<usize>()
}

fn determine_display_value(data: &str) -> usize {
    let registers = data.split(" | ").next().unwrap()
        .split(' ')
        .map(|x| x.chars().collect::<HashSet<char>>())
        .collect::<Vec<HashSet<char>>>();
    let translation = determine_translation(&registers);
    data.split(" | ").nth(1).unwrap()
        .split(' ')
        .map(|x| {
            let mut v = x.chars().collect::<Vec<char>>();
            v.sort_unstable();
            translation.get(&v.iter().copied().collect::<String>())
                .unwrap()
        })
        .fold(0, |s: usize, i| 10 * s + i)
}

fn char_id(data: &HashSet<char>) -> String {
    let mut char_vec = data.iter()
        .copied()
        .collect::<Vec<char>>();
    char_vec.sort_unstable();
    char_vec.iter().copied().collect::<String>()
}

fn determine_translation(extract: &[HashSet<char>]) -> HashMap<String, usize> {
    let mut res = HashMap::new();
    // Extract the known digits
    // Digit one : One
    let one = extract.iter()
        .find(|x| x.len() == 2)
        .unwrap().iter().copied()
        .collect::<HashSet<char>>();
    res.insert(char_id(&one), 1);

    // Digit two : Four
    let four = extract.iter()
        .find(|x| x.len() == 4)
        .unwrap().iter().copied()
        .collect::<HashSet<char>>();
    res.insert(char_id(&four), 4);

    // Digit three : Seven
    let seven = extract.iter()
        .find(|x| x.len() == 3)
        .unwrap().iter().copied()
        .collect::<HashSet<char>>();
    res.insert(char_id(&seven), 7);

    // Digit three : Eight
    let eight = extract.iter()
        .find(|x| x.len() == 7)
        .unwrap().iter().copied()
        .collect::<HashSet<char>>();
    res.insert(char_id(&eight), 8);

    // Find segment A
    let segment_a = seven.difference(&one).next().unwrap();
    // Find segment G
    let mut segment_g = extract.iter()
        .filter(|x| x.len() == 5 || x.len() == 6)
        .fold(None, |s: Option<HashSet<char>>, i| {
            let hs = i.iter().copied().collect();
            s.map(|a| a.intersection(i).copied().collect())
                .or(Some(hs))
        }).unwrap();
    segment_g.remove(segment_a);
    let segment_g = segment_g.iter().next().unwrap();

    // Find segment E
    let mut segment_e = HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']);
    segment_e = segment_e.difference(&four).copied()
        .collect::<HashSet<char>>();
    segment_e.remove(segment_a);
    segment_e.remove(segment_g);
    let segment_e = segment_e.iter().next().unwrap();

    // We now have enough segments
    // We can get 9
    let mut nine = HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']);
    nine.remove(segment_e);
    res.insert(char_id(&nine), 9);

    // The last one present with 6 segments, where 1 is not subset, is 6
    let six = extract.iter()
        .find(|&x| x.len() == 6 && *x != nine && !one.is_subset(x))
        .unwrap();
    res.insert(char_id(six), 6);

    // The last one present with six segments is 0
    let zero = extract.iter()
        .find(|&x| x.len() == 6 && *x != nine && x != six)
        .unwrap();
    res.insert(char_id(zero), 0);
    
    // Two is the only five segment number with segment E
    let two = extract.iter()
        .find(|&x| x.len() == 5 && x.contains(segment_e))
        .unwrap();
    res.insert(char_id(two), 2);

    // Three has a full intersection with 7
    let three = extract.iter()
        .find(|&x| x.len() == 5 && seven.is_subset(x))
        .unwrap();
    res.insert(char_id(three), 3);

    // Five is the last one
    let five = extract.iter()
        .find(|&x| x.len() == 5 && x != two && x != three)
        .unwrap();
    res.insert(char_id(five), 5);

    res
}
