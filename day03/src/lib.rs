//! Module containing the logic used to solve advent of code day 03

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

fn get_common(data: &Vec<Vec<usize>>, pos: usize) -> usize {
    let mid: f64 = (data.len() as f64)/2_f64;
    if data.iter()
        .map(|x| *x.get(pos).unwrap() as f64)
        .sum::<f64>() >= mid
        {
            1
        } else {
            0
        }
}

fn get_number(data: &Vec<Vec<usize>>, is_generator: bool) -> usize {
    let numsize = data.get(0).unwrap().len();
    let mut potential = data.clone();
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
