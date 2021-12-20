//! Library module with all the logic

use std::collections::{HashMap, HashSet};
use regex::Regex;

type Coords = (isize, isize, isize);

/// Solve Advent of Code day 19 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 19.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
    let re_scan: Regex = Regex::new(r"^--- scanner (\d+) ---$").unwrap();
    let re_beac: Regex = Regex::new(r"^(-?\d+),(-?\d+),(-?\d+)$").unwrap();
    let mut vscans: HashMap<usize, Scanner> = data.trim().split("\n\n")
        .map(|scanline| {
            let lines = scanline.split('\n').collect::<Vec<&str>>();
            let k = re_scan.captures(lines[0]).unwrap()
                .get(1).unwrap().as_str();
            let beacs = lines[1..].iter()
                .map(|line| {
                    let k = re_beac.captures(line).unwrap();
                    let k = (1..=3)
                        .map(|i| k.get(i).unwrap()
                             .as_str().parse::<isize>().unwrap())
                        .collect::<Vec<isize>>();
                    (k[0], k[1], k[2])
                })
                .collect::<Vec<Coords>>();
            (k.parse::<usize>().unwrap(),
                Scanner {
                    positioned: false,
                    x: 0, y: 0, z: 0,
                    rot: [
                        [0, 0, 0],
                        [0, 0, 0],
                        [0, 0, 0]
                    ],
                    beacons: beacs
                })
        })
        .collect::<HashMap<usize, Scanner>>();
    // Consider that the first Scanner is absolutely placed
    let scan_zero: &mut Scanner = vscans.get_mut(&0).unwrap();
    scan_zero.positioned = true;
    scan_zero.rot = [
        [1, 0, 0],
        [0, 1, 0],
        [0, 0, 1]
    ];
    // Try and correlate them all to each other
    let mut stables = vec![0];
    let mut unstables = (1..vscans.len()).collect::<Vec<usize>>();
    while !unstables.is_empty() {
        let i = unstables[0];
        //println!("Picking {} from {:?}", i, unstables);
        for stable in &stables {
            //println!("Trying to correlate {} and {}...", stable, i);
            let beac_stable = vscans.get(stable).unwrap().clone();
            if beac_stable.correlate(vscans.get_mut(&i).unwrap()) {
                // Rework this second guy
                //println!("Successfully correlated {} and {}", stable, i);
                //let scanner_mod: &mut Scanner = vscans.get_mut(&i).unwrap();
                //println!("Location of {} is {:?} in absolute", i,
                //         (scanner_mod.x, scanner_mod.y, scanner_mod.z));
                stables.push(i);
                unstables.remove(0);
                //println!("Unstables left: {:?}", unstables);
                break;
            }
        }
        //println!("Trying to correlate someone else...");
        if !unstables.is_empty() && i == unstables[0] {
            unstables.push(i);
            unstables.remove(0);
        }
    }
    // Collect
    let beacons: HashSet<Coords> = vscans.iter()
        .flat_map(|(_, x)| x.beacons.clone())
        .collect::<HashSet<Coords>>();
    // Count
    beacons.len()
}

#[derive(Clone)]
struct Scanner {
    positioned: bool,
    x: isize,
    y: isize,
    z: isize,
    rot: [[isize; 3]; 3], // Rotation matrix
    beacons: Vec<Coords>
}

impl Scanner {
    //fn in_range(&self, x: isize, y: isize, z: isize) -> bool {
        //(x - self.x).abs() < 1000 && (y - self.y).abs() < 1000 && (z - self.z).abs() < 1000
    //}

    fn correlate(&self, other: &mut Scanner) -> bool {
        // One beacon at a time, try and map
        for &head_anch in &self.beacons {
            for &head_float in &other.beacons {
                // Map head_anch -> head_float
                //println!("Trying to tie {:?} to {:?}...", head_anch, head_float);
                // Pick a second pair
                for &aux_anch in &self.beacons {
                    if aux_anch == head_anch { continue; }
                    for &aux_float in &other.beacons {
                        if aux_float == head_float { continue; }
                        let mut mapped: Vec<(Coords, Coords)> =
                            vec![(head_anch, head_float), (aux_anch, aux_float)];
                        // So if these two pairs correspond.. Can they?
                        if distance(head_anch, aux_anch) !=
                            distance(head_float, aux_float) { continue; }
                        let dist_no_rot = sub_coords(head_anch, aux_anch);
                        let dist_w_rot = sub_coords(head_float, aux_float);
                        let rot = induce_rotation(dist_no_rot, dist_w_rot);
                        if rot == [[0; 3]; 3] { continue; }
                        //println!("Potential: {:?} and {:?}", aux_anch,
                                 //aux_float);
                        //println!("Rotare: {:?} and {:?}", dist_no_rot,
                                 //dist_w_rot);
                        //println!("Deduced rotation: {:?}", rot);
                        // Find out the translation
                        let trans_between = sub(head_anch, mul(rot, head_float));
                        for &att_float in &other.beacons {
                            // Apply the thingies
                            let att_tr_float = add(trans_between, mul(rot, att_float));
                            // Check existence
                            if self.beacons.contains(&att_tr_float) {
                                mapped.push((att_tr_float, att_float));
                            }
                            if mapped.len() > 12 {
                                // Do the transformations here
                                other.positioned = true;
                                other.rot = rot;
                                other.beacons = other.beacons.iter()
                                    .map(|&x| add(trans_between, mul(rot, x)))
                                    .collect::<Vec<Coords>>();
                                other.x = trans_between.0;
                                other.y = trans_between.1;
                                other.z = trans_between.2;
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }
}

fn add(a: Coords, b: Coords) -> Coords {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn sub(a: Coords, b: Coords) -> Coords {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn mul(rot: [[isize; 3]; 3], v: Coords) -> Coords {
    (
        v.0 * rot[0][0] + v.1 * rot[0][1] + v.2 * rot[0][2],
        v.0 * rot[1][0] + v.1 * rot[1][1] + v.2 * rot[1][2],
        v.0 * rot[2][0] + v.1 * rot[2][1] + v.2 * rot[2][2]
    )
}

fn induce_rotation(a: Coords, b: Coords) -> [[isize; 3]; 3] {
    // So
    if a.0.abs() == a.1.abs() || a.1.abs() == a.2.abs() || a.0.abs() == a.2.abs() { // No, I can't guess two numbers at once, ffs
        return [[0; 3]; 3]; }
    match (
        a.0.abs() == b.0.abs(),
        a.0.abs() == b.1.abs(),
        a.0.abs() == b.2.abs()
        ) {
        (true, false, false) =>
            match (a.1.abs() == b.1.abs(), a.1.abs() == b.2.abs()) {
                (true, false) => [
                    [a.0/b.0, 0, 0],
                    [0, a.1/b.1, 0],
                    [0, 0, a.2/b.2]
                ],
                (false, true) => [
                    [a.0/b.0, 0, 0],
                    [0, 0, a.1/b.2],
                    [0, a.2/b.1, 0]
                ],
                _ => [[0; 3]; 3]
            },
        (false, true, false) =>
            match (a.1.abs() == b.0.abs(), a.1.abs() == b.2.abs()) {
                (true, false) => [
                    [0, a.0/b.1, 0],
                    [a.1/b.0, 0, 0],
                    [0, 0, a.2/b.2]
                ],
                (false, true) => [
                    [0, a.0/b.1, 0],
                    [0, 0, a.1/b.2],
                    [a.2/b.0, 0, 0]
                ],
                _ => [[0; 3]; 3]
            },
        (false, false, true) =>
            match (a.1.abs() == b.0.abs(), a.1.abs() == b.1.abs()) {
                (true, false) => [
                    [0, 0, a.0/b.2],
                    [a.1/b.0, 0, 0],
                    [0, a.2/b.1, 0]
                ],
                (false, true) => [
                    [0, 0, a.0/b.2],
                    [0, a.1/b.1, 0],
                    [a.2/b.0, 0, 0]
                ],
                _ => [[0; 3]; 3]
            },
        _ => [[0; 3]; 3]
    }
}

fn sub_coords(a: Coords, b: Coords) -> Coords {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn distance(a: Coords, b: Coords) -> isize {
    sq(a.0 - b.0) +
        sq(a.1 - b.1) +
        sq(a.2 - b.2)
}

fn sq(i: isize) -> isize { i*i }

/// Solve Advent of Code day 19 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 19.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> isize {
    let re_scan: Regex = Regex::new(r"^--- scanner (\d+) ---$").unwrap();
    let re_beac: Regex = Regex::new(r"^(-?\d+),(-?\d+),(-?\d+)$").unwrap();
    let mut vscans: HashMap<usize, Scanner> = data.trim().split("\n\n")
        .map(|scanline| {
            let lines = scanline.split('\n').collect::<Vec<&str>>();
            let k = re_scan.captures(lines[0]).unwrap()
                .get(1).unwrap().as_str();
            let beacs = lines[1..].iter()
                .map(|line| {
                    let k = re_beac.captures(line).unwrap();
                    let k = (1..=3)
                        .map(|i| k.get(i).unwrap()
                             .as_str().parse::<isize>().unwrap())
                        .collect::<Vec<isize>>();
                    (k[0], k[1], k[2])
                })
                .collect::<Vec<Coords>>();
            (k.parse::<usize>().unwrap(),
                Scanner {
                    positioned: false,
                    x: 0, y: 0, z: 0,
                    rot: [
                        [0, 0, 0],
                        [0, 0, 0],
                        [0, 0, 0]
                    ],
                    beacons: beacs
                })
        })
        .collect::<HashMap<usize, Scanner>>();
    // Consider that the first Scanner is absolutely placed
    let scan_zero: &mut Scanner = vscans.get_mut(&0).unwrap();
    scan_zero.positioned = true;
    scan_zero.rot = [
        [1, 0, 0],
        [0, 1, 0],
        [0, 0, 1]
    ];
    // Try and correlate them all to each other
    let mut stables = vec![0];
    let mut unstables = (1..vscans.len()).collect::<Vec<usize>>();
    let mut beacon_pos: Vec<Coords> = Vec::new();
    while !unstables.is_empty() {
        let i = unstables[0];
        //println!("Picking {} from {:?}", i, unstables);
        for stable in &stables {
            //println!("Trying to correlate {} and {}...", stable, i);
            let beac_stable = vscans.get(stable).unwrap().clone();
            if beac_stable.correlate(vscans.get_mut(&i).unwrap()) {
                // Rework this second guy
                //println!("Successfully correlated {} and {}", stable, i);
                let scanner_found: &Scanner = vscans.get(&i).unwrap();
                //println!("Location of {} is {:?} in absolute", i,
                //         (scanner_mod.x, scanner_mod.y, scanner_mod.z));
                stables.push(i);
                unstables.remove(0);
                //println!("Unstables left: {:?}", unstables);
                beacon_pos.push((scanner_found.x, scanner_found.y, scanner_found.z));
                break;
            }
        }
        //println!("Trying to correlate someone else...");
        if !unstables.is_empty() && i == unstables[0] {
            unstables.push(i);
            unstables.remove(0);
        }
    }
    // Take the collected positions
    let mut dist_max = 0;
    for i in 0..beacon_pos.len() {
        for j in 0..i {
            if i == j { continue; }
            let m = manhattan(beacon_pos[i], beacon_pos[j]);
            dist_max = if m > dist_max { m } else { dist_max };
        }
    }
    dist_max
}

fn manhattan(a: Coords, b: Coords) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()
}
