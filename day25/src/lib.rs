//! Library module with all the logic

use std::collections::HashSet;

type Coords = (usize, usize);

/// Solve Advent of Code day 25 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 25.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
    let mut p = 1;
    let mut sfloor = data.parse::<SeaFloor>().unwrap();
    loop {
        //println!("\nStep {}\n{}", p, sfloor);
        if !sfloor.single_step() {
            break;
        }
        p += 1;
    }
    p
}

struct SeaFloor {
    floor_s: HashSet<(usize, usize)>,
    floor_e: HashSet<Coords>,
    max: Coords
}

impl SeaFloor {
    fn single_step(&mut self) -> bool {
        let mut moved: bool = false;
        let mut rem_floor: HashSet<Coords> = HashSet::new();
        let mut add_floor: HashSet<Coords> = HashSet::new();
        // Move the eastward cucumbers
        for &(y, x) in &self.floor_e {
            let next_coord: Coords = (y, (x+1)%self.max.1);
            if !self.floor_e.contains(&next_coord) &&
                !self.floor_s.contains(&next_coord) {
                moved = true;
                add_floor.insert(next_coord);
                rem_floor.insert((y, x));
            }
        }
        for c in rem_floor.drain() { self.floor_e.remove(&c); }
        self.floor_e.extend(add_floor.clone());
        add_floor.clear();

        // Move the southward cucumbers
        for &(y, x) in &self.floor_s {
            let next_coord: Coords = ((y+1)%self.max.0, x);
            // Fun fact : most cucumbers are likely to bump into
            // Someone of their own group. So, having the southbound
            // Cucumbers checked first actually lets us skip a lot of
            // Checking.
            if !self.floor_s.contains(&next_coord) &&
                !self.floor_e.contains(&next_coord) {
                moved = true;
                add_floor.insert(next_coord);
                rem_floor.insert((y, x));
            }
        }
        for c in rem_floor { self.floor_s.remove(&c); };
        self.floor_s.extend(add_floor);
        moved
    }
}

impl std::fmt::Display for SeaFloor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut st: String = String::new();
        for y in 0..self.max.0 {
            for x in 0..self.max.1 {
                st.push(match (self.floor_s.get(&(y, x)).is_some(), self.floor_e.get(&(y,x)).is_some()) {
                    (false, false) => '.',
                    (false, true) => '>',
                    (true, false) => 'v',
                    (true, true) => panic!()
                });
            }
            if y != self.max.0-1 {
                st.push('\n');
            }
        }
        write!(f, "{}", st)
    }
}

impl std::str::FromStr for SeaFloor {
    type Err = SeaFloorError;
    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let mut floor_s: HashSet<Coords> = HashSet::new();
        let mut floor_e: HashSet<Coords> = HashSet::new();
        let mut max_y: usize = 0;
        let mut max_x: usize = 0;
        for (y, line) in data.trim().split('\n').enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '>' => { floor_e.insert((y, x)); },
                    'v' => { floor_s.insert((y, x)); },
                    '.' => { /* normal see floor */ },
                    _ => return Err(SeaFloorError {})
                }
                if max_y == 0 {
                    max_x += 1;
                }
            }
            max_y += 1;
        }
        Ok(SeaFloor { floor_e, floor_s, max: (max_y, max_x) })
    }
}

#[derive(Debug)]
struct SeaFloorError { }
impl std::fmt::Display for SeaFloorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bleh")
    }
}
impl std::error::Error for SeaFloorError {}


/// Solve Advent of Code day 25 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// It returns 0 as [usize] to celebrate.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(_: &str) -> String {
    // Merry Christmas
    "Merry Christmas!".into()
}

