//! Module for utility definitions

// Some types
pub type Coords = (isize, isize, isize);
pub type Cuboid = (Coords, Coords);

pub fn intersection(a: Cuboid, b: Cuboid) -> Option<Cuboid> {
    // No intersection
    let min_of_max = (a.1.0.min(b.1.0), a.1.1.min(b.1.1), a.1.2.min(b.1.2));
    let max_of_min = (a.0.0.max(b.0.0), a.0.1.max(b.0.1), a.0.2.max(b.0.2));
    if max_of_min.0 > min_of_max.0 ||
        max_of_min.1 > min_of_max.1 ||
            max_of_min.2 > min_of_max.2 { None }
    else {
        Some((max_of_min, min_of_max))
    }
}

pub fn split_cubes(full: Cuboid, incube: Cuboid) -> Vec<Cuboid> {
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
                        cubes.push((
                                (lb_x, lb_y, lb_z),
                                (hb_x, hb_y, hb_z)
                                   ));
                    }
            }
    }
    cubes.into_iter()
        .filter(|&c| volume(c) > 0)
        .collect::<Vec<Cuboid>>()
}

pub fn volume(c: Cuboid) -> usize {
    if c.1.0 < c.0.0 || c.1.1 < c.0.1 || c.1.2 < c.0.2 { 0 }
    else {
        (c.1.0 - c.0.0 + 1).unsigned_abs() *
            (c.1.1 - c.0.1 + 1).unsigned_abs() *
            (c.1.2 - c.0.2 + 1).unsigned_abs()
    }
}

