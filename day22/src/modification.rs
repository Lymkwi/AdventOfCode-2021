//! Module defining a modification rule
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE_MOD: Regex = Regex::new(r"^(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$").unwrap();
}

use crate::utils::{Coords, Cuboid};

/// Modification rule
///
/// A [Modification] rule describes a [Cuboid] area and the boolean
/// corresponding to its effect (`true` for "on", `false` for "false").
#[derive(Debug)]
pub struct Modification {
    kind: bool,
    // I could have used a `Cuboid` here but that type didn't
    // Exist when I wrote this
    x_range: (isize, isize),
    y_range: (isize, isize),
    z_range: (isize, isize)
}

impl Modification {
    /// Is the provided set of coordinates within our [Cuboid] ?
    ///
    /// Remember that the ranges are inclusive.
    #[must_use]
    pub fn is_within(&self, a: Coords) -> bool {
        self.x_range.0 <= a.0 && a.0 <= self.x_range.1 &&
            self.y_range.0 <= a.1 && a.1 <= self.y_range.1 &&
            self.z_range.0 <= a.2 && a.2 <= self.z_range.1
    }

    /// Fetch the kind of operation for this modification
    ///
    /// # Return Value
    ///
    /// A `true` boolean corresponds to "on", and a `false` to "off".
    #[must_use]
    pub fn is_on(&self) -> bool {
        self.kind
    }

    /// Fetch the [Cuboid] for this rule
    #[must_use]
    pub fn get_cube(&self) -> Cuboid {
        (
            (self.x_range.0, self.y_range.0, self.z_range.0),
            (self.x_range.1, self.y_range.1, self.z_range.1)
        )
    }
}

/// Error thrown when parsing an incorrect string into
/// a [Modification].
#[derive(Debug)]
pub struct ParseModificationError { }
impl std::fmt::Display for ParseModificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error unparsing rule!!")
    }
}
impl std::error::Error for ParseModificationError { }

impl std::str::FromStr for Modification {
    type Err = ParseModificationError;
    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let caps = &RE_MOD.captures(data).ok_or(ParseModificationError {  })?;
        let caps = &caps.iter()
            // We skip caps[0] because that is the whole string.
            .skip(1)
            .map(|x| x.unwrap().as_str())
            .collect::<Vec<&str>>()[0..7];
        // Now, caps[0] is the type
        let kind = caps[0] == "on";
        // All the rest is numbers for the `Cuboid` of the rule
        let caps = caps.iter().skip(1)
            // This line will technically panic instead of
            // Yielding a `ParseModificationError` but who
            // Cares, it'll never panic.
            .map(|&x| x.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        Ok(Modification {
            kind,
            x_range: (caps[0], caps[1]),
            y_range: (caps[2], caps[3]),
            z_range: (caps[4], caps[5])
        })
    }

}

