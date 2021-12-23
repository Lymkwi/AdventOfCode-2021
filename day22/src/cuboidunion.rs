//! Module defining a [`CuboidUnion`] that keeps track of the union of
//! multiple cuboids.

use crate::utils::{Cuboid, intersection, volume, split_cubes};

/// Representation of a union of many cuboids
///
/// Internally, this is just a wrapper for a [Vec] of [Cuboid].
/// Historically, it wasn't always, but I find the implementation
/// of the structure fairly useful.
pub struct CuboidUnion {
    cuboids: Vec<Cuboid>
}

impl CuboidUnion {
    /// Add a cuboid to the set represented by the Union
    ///
    /// # Return Value
    ///
    /// Returns the difference in volume that the addition of the
    /// provided [Cuboid] creates.
    ///
    /// # Mechanism
    ///
    /// In order to add a [Cuboid] to the current covering set,
    /// you first begin by putting this [Cuboid] in a [Vec].
    /// Then, for all cuboids currently in the covering set,
    /// you iterate once through the entire vector. At first,
    /// there is only one [Cuboid] in there. You compute the
    /// intersection of the [Cuboid] you're adding, and the
    /// one you are iterating over from the covering set.
    /// If the intersection is empty, nothing happens. If
    /// the intersection happens to be a [Cuboid], it means
    /// you need to split up the [Cuboid] you are going to add
    /// into several pieces that do not overlap with the current
    /// covering set. Once that is done, you take all these pieces,
    /// and add them to the [Vec] we started with.
    ///
    /// When all of the [Cuboid]s previously present in the covering
    /// set have been iterated over, you end up with a [Vec] of cuboids
    /// that are at the same time sub-cuboids of the one we wanted to
    /// add, but that have strictly no intersection with any [Cuboid]
    /// already in the union.
    ///
    /// This mechanism works, and we do not have to begin iterating
    /// all over again for each new sub-cuboid because the intersection
    /// of an already added cuboid and the new one is systematically
    /// removed after breaking the larger cuboid, meaning that the
    /// sub-cuboids we preserve have never intersected with any of the
    /// previous cuboids known from the union.
    ///
    /// # Panics
    ///
    /// There is a potential for this method to panic in a situation
    /// Where, for some reason, upon splitting a [Cuboid] into many
    /// sub-cuboids, the intersection cuboid is not present.
    ///
    /// It should never happen, but if it does, know that something
    /// is deeply broken.
    #[must_use]
    pub fn add(&mut self, c: Cuboid) -> usize {
        // And now remove the intersections
        let mut cuboids: Vec<Cuboid> = vec![c];
        for &old_c in &self.cuboids {
            let mut new_cuboids = Vec::new();
            for c in cuboids {
                // Compute the intersection of this cuboid and the
                // Current cuboid known from the union
                if let Some(intercube) = intersection(old_c, c) {
                    // If the intersection is itself, discard this
                    // Cuboids. Not adding it to `new_cuboids`
                    // Effectively does so.
                    if intercube == c { continue; }
                    let mut ncubes = split_cubes(c, intercube);
                    // This assert used to check the logic
                    // Of the split_cubes method, which is
                    // How I figured out that I had switched
                    // Two numbers in that method that messed
                    // Up my results.
                    //assert_eq!(volume(c), ncubes.iter()
                               //.map(|&x| volume(x))
                               //.sum::<usize>());
                    // We remove precisely the part of the
                    // original cuboid `c` that intersects with
                    // existing regions.
                    let intercubepos = ncubes.iter()
                        .position(|&k| k == intercube).unwrap();
                    ncubes.remove(intercubepos);
                    // Then, we push all of these new cuboids for
                    // The next iteration over known cuboids in the
                    // Union.
                    new_cuboids.extend(ncubes);
                } else {
                    // If there was no intersection you can just
                    // Leave the cuboid as is.
                    new_cuboids.push(c);
                }
            }
            // Shortcut : no cuboids here means that all of what
            // We wanted to add was already in the set. If that
            // Was the case then the delta of volume is 0 and we
            // Don't need to keep iterating over the rest of the
            // Known cuboids from the union.
            if new_cuboids.is_empty() { return 0; }
            cuboids = new_cuboids;
        }
        let nvol = cuboids.iter()
            .map(|&x| volume(x))
            .sum::<usize>();
        // This check was more useful for debugging
        // Than you may think it was
        //assert!(nvol <= volume(c));
        // Add the cuboids that never intersected with anything
        // To the list of cuboids from the known union. That way,
        // That list always contains mutually non-intersecting cuboids.
        self.cuboids.extend(cuboids);
        nvol // Exclusive volume added by `c` (i.e. delta of volume)
    }

    /// Create a new [`CuboidUnion`] that is empty
    #[must_use]
    pub fn new() -> CuboidUnion {
        CuboidUnion { cuboids: Vec::new() }
    }
}

impl Default for CuboidUnion {
    fn default() -> Self {
        Self::new()
    }
}

