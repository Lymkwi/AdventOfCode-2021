//! Library module with all the logic

/// Solve Advent of Code day 01 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input formatted
///  as such : `<depth integer>\n` an unknown amount of times.
///
/// # Return value
///
/// This function returns a `Result<usize,()>` where `Ok` holds the result for part
/// one of advent of code day 01.
///
/// # Errors
///
/// There is no custom error type here so `Err` always contains `()`.
pub fn solve_part_one(data: &str) -> Result<usize,()> {
    // Build the data into a Vec<i32>
    Ok(data.trim().split('\n')
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .as_slice()
        .windows(2)
        .fold(0, |c, i| if i[0] < i[1] { c + 1 } else { c }))
}

/// Solve Advent of Code day 01 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input formatted
///  as such : `<depth integer>\n` an unknown amount of times.
///
/// # Return value
///
/// This function returns a `Result<usize,()>` where `Ok` holds the result for part
/// two of advent of code day 01.
///
/// # Errors
///
/// There is no custom error type here so `Err` always contains `()`.
pub fn solve_part_two(data: &str) -> Result<usize,()> {
    Ok(data.trim().split('\n')
       .map(|v| v.parse::<i32>().unwrap())
       .collect::<Vec<i32>>()
       .as_slice()
       .windows(3)
       .map(|x| x[0] + x[1] + x[2])
       .collect::<Vec<i32>>()
       .as_slice()
       .windows(2)
       .fold(0, |c, i| if i[0] < i[1] { c + 1 } else { c }))
}


