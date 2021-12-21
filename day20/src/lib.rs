//! Library module with all the logic

use std::collections::HashSet;

type Program = Vec<bool>;
type Coords = (isize, isize);

/// Solve Advent of Code day 20 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 20.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_one(data: &str) -> usize {
    let mut machine: Enhancer = Enhancer::unparse(data);
    //println!("Bounds: {:?}", machine.get_bounds());
    for _ in 0..2 {
        //println!("--- Step {:02} ---", x);
        //println!("{}", machine);
        machine.step();
    }
    //println!("--- Step Final ---");
    //println!("{}", machine);
    assert!(!machine.exterior);
    machine.count()
}

struct Enhancer {
    data: HashSet<Coords>,
    exterior: bool,
    prog: Program,
    bounds_mem: (Coords, Coords)
}

impl std::fmt::Display for Enhancer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut st: String = String::new();
        let extc: char = if self.exterior { '#' } else { '.' };
        // Get bounds
        let ((y_min, x_min), (y_max, x_max)) = self.bounds_mem;
        for y in (y_min-1)..=(y_max+1) {
            for x in (x_min-1)..=(x_max+1) {
                if x < x_min || x > x_max || y < y_min || y > y_max {
                    st.push(extc);
                    continue;
                }
                st.push(if self.data.contains(&(y, x)) { '#' } else { '.' });
            }
            st.push('\n');
        }
        write!(f, "{}", st)
    }
}

impl Enhancer {
    fn unparse(data: &str) -> Enhancer {
        let splits = data.split("\n\n").collect::<Vec<&str>>();
        let prog: Program = splits.get(0).unwrap()
            .chars()
            .map(|x| match x {
                '.' => false,
                '#' => true,
                x => panic!("Unknown char '{}' in program", x)
            })
            .collect::<Program>();
        let data = splits.get(1).unwrap()
            .split('\n')
            .enumerate()
            .flat_map(|(y, line)| line.chars()
                 .enumerate()
                 .filter_map(|(x, c)| if c == '#' {
                     Some((
                             isize::try_from(y).unwrap(),
                             isize::try_from(x).unwrap()
                             )) 
                 } else { None })
                 .collect::<Vec<Coords>>()
            )
            .collect::<HashSet<Coords>>();
        let mut res = Enhancer { data, prog,
            exterior: false, bounds_mem: ((0, 0), (0, 0)) };
        res.update_bounds();
        res
    }

    fn count(&self) -> usize {
        self.data.len()
    }

    fn update_bounds(&mut self) {
        self.bounds_mem = self.data.iter()
            .fold(((0, 0), (0, 0)),
            |s, &(a, b)| (
                (a.min(s.0.0), b.min(s.0.1)), // min
                (a.max(s.1.0), b.max(s.1.1))  // max
                         ));
    }

    fn step(&mut self) {
        // First, copy the data
        let ((y_min, x_min), (y_max, x_max)) = self.bounds_mem;
        let pixelgrowth = [
            (1, 1), (1, 0), (1, -1),
            (0, 1) , (0, 0),  (0, -1),
            (-1, 1), (-1, 0), (-1, -1)];
        let mut nbounds = ((0, 0), (0, 0));
        self.data = ((y_min-1)..=(y_max+1))
            .flat_map(|y| ((x_min-1)..=(x_max+1))
                 .filter_map(|x| {
                     // Determine whether or not this pixel stays on
                     let mut prog_coords = 0;
                     for (c, dpos) in pixelgrowth.iter().enumerate() {
                         if self.contains((y+dpos.0, x+dpos.1)) {
                             prog_coords += 2_usize.pow(c.try_into().unwrap());
                         }
                     }
                     if self.prog[prog_coords] {
                         // Compute min
                         nbounds.0.0 = y.min(nbounds.0.0);
                         nbounds.0.1 = x.min(nbounds.0.1);
                         nbounds.1.0 = y.max(nbounds.1.0);
                         nbounds.1.1 = x.max(nbounds.1.1);
                         Some((y, x))
                     } else { None }
                 })
                 .collect::<Vec<Coords>>())
            .collect::<HashSet<Coords>>();
        self.bounds_mem = nbounds;
        // Now update the exterior bounds
        self.exterior = self.prog[if self.exterior { 511 } else { 0 }];
    }

    fn contains(&self, a: Coords) -> bool {
        if a.0 < self.bounds_mem.0.0 || a.0 > self.bounds_mem.1.0 ||
            a.1 < self.bounds_mem.0.1 || a.1 > self.bounds_mem.1.1
            { self.exterior }
        else { self.data.contains(&a) }
    }
}

/// Solve Advent of Code day 20 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds both numbers for today's input.
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 20.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
pub fn solve_part_two(data: &str) -> usize {
    let mut machine: Enhancer = Enhancer::unparse(data);
    for _ in 0..50 {
        machine.step();
    }
    //println!("--- Step Final ---");
    //println!("{}", machine);
    assert!(!machine.exterior);
    machine.count()
}

