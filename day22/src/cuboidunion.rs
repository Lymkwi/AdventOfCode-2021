//! Module defining a `CuboidUnion` that keeps track of the union of
//! multiple cuboids.

use crate::utils::{Cuboid, intersection, volume, split_cubes};

pub struct CuboidUnion {
    cuboids: Vec<Cuboid>
}

impl CuboidUnion {
    pub fn add(&mut self, c: Cuboid) -> usize {
        // And now remove the intersections
        let mut cuboids: Vec<Cuboid> = vec![c];
        for &old_c in &self.cuboids {
            let mut new_cuboids = Vec::new();
            for c in cuboids {
                // Compute the intersection of this cuboid and the current cube
                if let Some(intercube) = intersection(old_c, c) {
                    if intercube == c { continue; }
                    let mut ncubes = split_cubes(c, intercube);
                    assert_eq!(volume(c), ncubes.iter()
                               .map(|&x| volume(x))
                               .sum::<usize>());
                    let intercubepos = ncubes.iter()
                        .position(|&k| k == intercube).unwrap();
                    ncubes.remove(intercubepos);
                    new_cuboids.extend(ncubes);
                } else {
                    new_cuboids.push(c);
                }
            }
            if new_cuboids.is_empty() { return 0; }
            cuboids = new_cuboids;
        }
        let nvol = cuboids.iter()
            .map(|&x| volume(x))
            .sum::<usize>();
        //assert!(nvol <= volume(c));
        self.cuboids.extend(cuboids);
        nvol // Exclusive volume
    }

    pub fn new() -> CuboidUnion {
        CuboidUnion { cuboids: Vec::new() }
    }
}


