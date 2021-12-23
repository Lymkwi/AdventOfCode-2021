//! Module for utility definitions

// Some types
/// Representation of the 3D coordinates of the engine
pub type Coords = (isize, isize, isize);
/// Representation of a 3D cuboid within the engine
pub type Cuboid = (Coords, Coords);

/// Determines a possible intersection between cuboids
///
/// # Arguments
///
/// This method is provided with two [Cuboid] objects.
///
/// # Return Value
///
/// Returns an [Option] of [Cuboid] which is `None` when there is
/// no intersection between the cuboids, and `Some(intersect)`
/// when the intersection between both provided cuboids is `intersect`.
#[must_use]
pub fn intersection(a: Cuboid, b: Cuboid) -> Option<Cuboid> {
    // It is easy to compute the bounds of the intersection cuboid
    // Its low-bound point is the maximum of the low-bound points,
    // Its high-bound point is the minimum of the high-bound points.
    let min_of_max = (a.1.0.min(b.1.0), a.1.1.min(b.1.1), a.1.2.min(b.1.2));
    let max_of_min = (a.0.0.max(b.0.0), a.0.1.max(b.0.1), a.0.2.max(b.0.2));
    // If either of these coordinates is in the wrong order it means there
    // Is no intersection.
    // Beware! These are strict equalities since our cube boundaries are
    // Inclusive. For example, we would have an intersection between
    // ((0, 0, 0), (1, 1, 1)) and ((0, 0, 0), (0, 0, 0)), which would be
    // the former cuboid.
    if max_of_min.0 > min_of_max.0 ||
        max_of_min.1 > min_of_max.1 ||
            max_of_min.2 > min_of_max.2 { None }
    else {
        // If the bounds we computed are valid for a cuboid, it is the
        // Intersection.
        Some((max_of_min, min_of_max))
    }
}

/// Splits a cuboid into up to 9 cuboids based on an intersecting cuboid
///
/// Given a cuboid that intersects with a larger cuboid, this method
/// returns a [Vec] of the cuboids obtained by splitting the larger one
/// into up to 9 cuboids (including the intersecting one). This method is
/// used to split a cuboid into smaller chunks that can be discarded if they
/// intersect with an existing set of cuboids.
///
/// # Arguments
///
///  - `full` is the larger, containing [Cuboid].
///  - `incube` is the smaller, intersecting [Cuboid].
///
/// # Return value
///
/// Returns a single [Vec] of [Cuboid] with all of the requested cuboids.
/// The size of that [Vec] can vary from `1` to `9` included.
///
/// # Implementation
#[must_use]
pub fn split_cubes(full: Cuboid, incube: Cuboid) -> Vec<Cuboid> {
    //! In order to generate the different cuboids, we begin by splitting
    //! the different axis into three region each :
    //! ```text
    //!   full.0                  full.1
    //!  ----|-----|----------|------|-->
    //!         incube.0   incube.1
    //! ```
    //!
    //! Then, for each axis, we take the boundaries of these regions.
    //! The combination of these boundaries for all three axis partitions
    //! the space of the cube perfectly into up to 9 non-intersecting cuboids.
    let mut cubes = Vec::new();
    for (lb_x, hb_x) in [
        (full.0.0, incube.0.0-1),
        (incube.0.0, incube.1.0),
        (incube.1.0+1, full.1.0)
    ] {
        for (lb_y, hb_y) in [
            (full.0.1, incube.0.1-1),
            (incube.0.1, incube.1.1),
            (incube.1.1+1, full.1.1)
        ] {
            for (lb_z, hb_z) in [
                (full.0.2, incube.0.2-1),
                (incube.0.2, incube.1.2),
                (incube.1.2+1, full.1.2)
            ] {
                cubes.push(((lb_x, lb_y, lb_z), (hb_x, hb_y, hb_z)));
            }
        }
    }
    // Filter out the potential cuboids that are empty/non-existent
    cubes.into_iter()
        .filter(|&c| volume(c) > 0)
        .collect::<Vec<Cuboid>>()
}

/// Compute the volume of a [Cuboid]
///
/// Note that the volume of a [Cuboid] like `((x, y, z), (x, y, z))`
/// is `1` (bounds are inclusive).
#[must_use]
pub fn volume(c: Cuboid) -> usize {
    if c.1.0 < c.0.0 || c.1.1 < c.0.1 || c.1.2 < c.0.2 { 0 }
    else {
        (c.1.0 - c.0.0 + 1).unsigned_abs() *
            (c.1.1 - c.0.1 + 1).unsigned_abs() *
            (c.1.2 - c.0.2 + 1).unsigned_abs()
    }
}

