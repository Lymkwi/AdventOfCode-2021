//! Module defining a modification rule
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE_MOD: Regex = Regex::new(r"^(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$").unwrap();
}

use crate::utils::{Coords, Cuboid};

#[derive(Debug)]
pub struct Modification {
    kind: bool,
    x_range: (isize, isize),
    y_range: (isize, isize),
    z_range: (isize, isize)
}

impl Modification {
    pub fn is_within(&self, a: Coords) -> bool {
        self.x_range.0 <= a.0 && a.0 <= self.x_range.1 &&
            self.y_range.0 <= a.1 && a.1 <= self.y_range.1 &&
            self.z_range.0 <= a.2 && a.2 <= self.z_range.1
    }

    pub fn is_on(&self) -> bool {
        self.kind
    }

    pub fn get_cube(&self) -> Cuboid {
        (
            (self.x_range.0, self.y_range.0, self.z_range.0),
            (self.x_range.1, self.y_range.1, self.z_range.1)
        )
    }

    pub fn from_str(data: &str) -> Modification {
        let caps = &RE_MOD.captures(data).unwrap().iter()
            .skip(1)
            .map(|x| x.unwrap().as_str())
            .collect::<Vec<&str>>()[0..7];
        let kind = caps[0] == "on";
        let caps = caps.iter().skip(1)
            .map(|&x| x.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        Modification {
            kind,
            x_range: (caps[0], caps[1]),
            y_range: (caps[2], caps[3]),
            z_range: (caps[4], caps[5])
        }
    }

}

