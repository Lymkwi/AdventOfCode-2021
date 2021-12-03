//! Module containing the logic used to solve advent of code day 03

/// Solve Advent of Code day 03 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, solution for part one of advent of code
/// day 03.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
    // Build a vec of vecs of ints to get our bits
    // into order
    let numbies = data.trim().split('\n')
        .map(|x| x.chars().map(|x| match x {
                '0' => 0,
                '1' => 1,
                e => panic!("Not a character I know: '{}'", e)
            })
            .collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();
    let numsize = numbies.get(0).unwrap().len();
    let midpoint = numbies.len()/2;
    // Then, sum it and see if the number of '1' in each column is above or
    // equal half of the total
    let sum = numbies.iter()
        .fold(vec![0; numsize], |state, n| {
                n.iter().enumerate()
                    .map(|(i, x)| state[i] + x )
                    .collect::<Vec<usize>>()
        });
    let bits_popular = sum.iter()
        .map(|&x| if x >= midpoint { '1' } else { '0' })
        .collect::<Vec<char>>();
    let epsilon = usize::from_str_radix(&bits_popular
                                        .iter().collect::<String>(), 2)
        .unwrap();
    let gamma = usize::from_str_radix(
        &bits_popular
            .iter()
            .map(|&x| if x == '1' { '0' } else { '1' })
            .collect::<String>(), 2).unwrap();
    gamma * epsilon
}

/// Solve Advent of Code day 03 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, solution for part two of advent of code
/// day 03.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
    // Build a vec of vecs of ints
    // It's much easier to do bit inspection later
    let numbies = data.trim().split('\n')
        .map(|x| x.chars().map(|x| match x {
                '0' => 0,
                '1' => 1,
                e => panic!("Not a character I know: '{}'", e)
            })
            .collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();
    let generator = get_number(&numbies, true);
    let scrubber = get_number(&numbies, false);
    generator * scrubber
}

fn get_common(data: &[Vec<usize>], pos: usize) -> usize {
    if data.iter()
        .map(|x| *x.get(pos).unwrap())
        .sum::<usize>() * 2 >= data.len()
        {
            1
        } else {
            0
        }
}

fn get_number(data: &[Vec<usize>], is_generator: bool) -> usize {
    let numsize = data.get(0).unwrap().len();
    let mut potential = data.to_owned();
    for i in 0..numsize {
        let common = get_common(&potential, i);
        potential = potential.into_iter()
            .filter(|x| if is_generator { common == *x.get(i).unwrap() }
                    else { common != *x.get(i).unwrap() })
            .collect();
        if potential.len() < 2 { break; }
    }
    usize::from_str_radix(&potential.get(0).unwrap()
        .iter().map(|&x| if x == 1 { '1' } else { '0' })
        .collect::<String>(), 2).unwrap()
}
