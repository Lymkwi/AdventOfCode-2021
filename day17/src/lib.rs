//! Library module with all the logic

use regex::Regex;

/// Solve Advent of Code day 17 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 17.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> isize {
    let re = Regex::new(r"^target area: x=(-?\d+)..(-?\d+), y=(-?\d+)\.\.(-?\d+)$").unwrap();
    let caps_vec = re.captures_iter(data.trim())
        .flat_map(|x|
            (1..5)
             .map(|n| x.get(n).unwrap().as_str())
             .collect::<Vec<&str>>())
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    if let [min_x, max_x, min_y, max_y] = caps_vec.get(..4).unwrap() {
        solve_no_mem((*min_x, *max_x), (*min_y, *max_y))
    } else { panic!("Bad format"); }
}

#[derive(std::fmt::Debug, PartialEq, Eq)]
enum Shot {
    Overshot,
    Undershot,
    Tunneled, // quantum leap
    Hit
}

fn solve_no_mem(x_zone: (isize, isize), y_zone: (isize, isize)) -> isize {
    // Some asserts
    let mut exp_y = 2_000; // this must be set too high
    let mut exp_x = 0;
    let mut exp_top_x = x_zone.1;
    assert_eq!(Shot::Undershot, big_shot(exp_x, exp_y, x_zone, y_zone));
    assert_eq!(Shot::Overshot, big_shot(exp_top_x, exp_y, x_zone, y_zone));
    // Look for tunneled
    let mut midx = (exp_x + exp_top_x)/2;
    loop {
        // Dichotomy
        match big_shot(midx, exp_y, x_zone, y_zone) {
            Shot::Hit | Shot::Tunneled => {
                exp_x = midx; break;
            },
            Shot::Undershot => {
                exp_x = midx;
            },
            Shot::Overshot => {
                exp_top_x = midx;
            }
        }
        midx = (exp_x + exp_top_x)/2;
    }
    //println!("potential x at {}", exp_x);
    // Walk back to the frontier of U/T
    while big_shot(exp_x, exp_y, x_zone, y_zone) == Shot::Tunneled {
        exp_x -= 1;
    }
    exp_x += 1;
    //println!("Final x is {}", exp_x);
    // Walk down to the border of H/T
    /*for y in (y_zone.0 .. 100).rev() {
        for x in (0..x_zone.1) {
            print!("{}", match big_shot(x, y, x_zone, y_zone) {
                Shot::Hit => "H",
                Shot::Undershot => "U",
                Shot::Overshot => "O",
                Shot::Tunneled => "T"
            });
        }
        println!();
    }*/
    // Walk down
    while big_shot(exp_x, exp_y, x_zone, y_zone) != Shot::Hit {
        exp_y -= 1;
    }
    //println!("Final y is {}", exp_y);
    (exp_y * (exp_y + 1))/2
}

/// Now is your chance to be a big shot
fn big_shot(x: isize, y: isize,
    x_zone: (isize, isize), y_zone: (isize, isize)) -> Shot {
    let (mut cx, mut cy): (isize, isize) = (0, 0);
    let (mut vx, mut vy): (isize, isize) = (x, y);
    loop {
        // Next step
        cx += vx; cy += vy;
        // Change velocity
        vx = match (vx > 0, vx < 0) {
            (true, _) => vx - 1,
            (_, true) => vx + 1,
            _ => 0
        };
        vy -= 1;
        // Detect ending
        //println!("{:?}", (cx, cy));
        // Are we in the zone
        if (x_zone.0 <= cx && cx <= x_zone.1) && (y_zone.0 <= cy && cy <= y_zone.1) {
            return Shot::Hit;
        } else if x_zone.0 <= cx && cx <= x_zone.1 && cy < y_zone.0 {
            return Shot::Tunneled;
        } else if vx == 0 && cx < x_zone.0 {
            return Shot::Undershot;
        } else if vx == 0 && cx > x_zone.1 {
            return Shot::Overshot;
        }
    }
}

/// Solve Advent of Code day 17 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 17.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> isize {
    let re = Regex::new(r"^target area: x=(-?\d+)..(-?\d+), y=(-?\d+)\.\.(-?\d+)$").unwrap();
    let caps_vec = re.captures_iter(data.trim())
        .flat_map(|x|
            (1..5)
             .map(|n| x.get(n).unwrap().as_str())
             .collect::<Vec<&str>>())
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    if let [min_x, max_x, min_y, max_y] = caps_vec.get(..4).unwrap() {
        scan_two((*min_x, *max_x), (*min_y, *max_y))
    } else { panic!("Bad format"); }
}

fn scan_two(x_zone: (isize, isize), y_zone: (isize, isize)) -> isize {
    // Some asserts
    let mut exp_y = 2_000; // this must be set high enough
    let mut exp_x = 0;
    let mut exp_top_x = x_zone.1;
    assert_eq!(Shot::Undershot, big_shot(exp_x, exp_y, x_zone, y_zone));
    assert_eq!(Shot::Overshot, big_shot(exp_top_x, exp_y, x_zone, y_zone));
    // Look for tunneled
    let mut midx = (exp_x + exp_top_x)/2;
    loop {
        // Dichotomy
        match big_shot(midx, exp_y, x_zone, y_zone) {
            Shot::Hit | Shot::Tunneled => {
                exp_x = midx; break;
            },
            Shot::Undershot => {
                exp_x = midx;
            },
            Shot::Overshot => {
                exp_top_x = midx;
            }
        }
        midx = (exp_x + exp_top_x)/2;
    }
    //println!("potential x at {}", exp_x);
    // Walk back to the frontier of U/T
    while big_shot(exp_x, exp_y, x_zone, y_zone) == Shot::Tunneled {
        exp_x -= 1;
    }
    exp_x += 1;
    //println!("Final x is {}", exp_x);
    // Walk down to the border of H/T
    /*for y in (y_zone.0 .. 100).rev() {
        for x in (0..x_zone.1) {
            print!("{}", match big_shot(x, y, x_zone, y_zone) {
                Shot::Hit => "H",
                Shot::Undershot => "U",
                Shot::Overshot => "O",
                Shot::Tunneled => "T"
            });
        }
        println!();
    }*/
    // Walk down
    while big_shot(exp_x, exp_y, x_zone, y_zone) != Shot::Hit {
        exp_y -= 1;
    }
    // Now with this information, walk along the path from undershot to
    // Overshot, for each y layer, until y reaches the y_zone.0, or
    // x reaches x_zone.1 without overshooting
    let mut count = 0;
    for y in (y_zone.0 ..= exp_y).rev() {
        for x in 0 ..=(x_zone.1+1) {
            match big_shot(x, y, x_zone, y_zone) {
                Shot::Hit => { count += 1; },
                _ => { }
            }
        }
    }
    count
}

