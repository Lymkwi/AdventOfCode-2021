/// Solve Advent of Code day 02 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input formatted
///  as such : `forward|up|down <number>\n` an unknown amount of times.
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
    let coords = data.trim().split('\n')
       .map(|v| {
           let toks = v.split(' ').collect::<Vec<&str>>();
           let delta: i32 = toks[1].parse::<i32>().unwrap();
           match toks.get(0).map(|&v| v) {
               Some("forward") => (0, delta),
               Some("down") => (delta, 0),
               Some("up") => (-delta, 0),
               Some(e) => panic!("Unknown pattern: '{}'", e),
               None => panic!("No token!")
           }
       })
       .fold((0, 0), |d, i| (d.0 + i.0, d.1 + i.1));
    (coords.0 * coords.1).try_into().map_err(|_| ())
}

/// Solve Advent of Code day 01 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input formatted
///  as such : `(up|down|forward) <integer>\n` an unknown amount of times.
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
    match data.trim().split('\n')
       .map(|v| {
           let toks = v.split(' ').collect::<Vec<&str>>();
           let delta: i32 = toks[1].parse::<i32>().unwrap();
           match toks.get(0).map(|&v| v) {
               Some("forward") => (0, delta),
               Some("down") => (delta, 0),
               Some("up") => (-delta, 0),
               Some(e) => panic!("Unknown pattern: '{}'", e),
               None => panic!("No token!")
           }
       })
       .fold((0, 0, 0), |data, order| {
           // order.1 changes the aim. You add it to the aim.
           // order.0 increases the hor by the value, and increases
           // depth by aim * value
           match order {
               (x, 0) => (data.0+x, data.1, data.2),
               (0, x) => (data.0, data.1+x, data.2+x*data.0),
               _ => panic!("Cannot happen")
           }
       }) {
           (_, x, y) => (x*y).try_into().map_err(|_| ())
       }
}

