use crate::LineSplitting;
use itertools::Itertools;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, PartialEq, Clone, Hash, Eq, Copy)]
pub struct Voxel(i64, i64, i64);

impl From<&str> for Voxel {
    fn from(value: &str) -> Self {
        let digits: Vec<i64> = value
            .splitn(3, ',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        Self(digits[0], digits[1], digits[2])
    }
}

pub trait Cube {
    fn neighbors(&self) -> NeighborsPosIterator;
}

impl Cube for Voxel {
    fn neighbors(&self) -> NeighborsPosIterator {
        NeighborsPosIterator {
            voxel: *self,
            face_idx: 0,
        }
    }
}

pub struct NeighborsPosIterator {
    voxel: Voxel,
    face_idx: u8,
}

//            +Y (4)
//            | -Z (3)
// (2) -X ____|/____+X (0)
//           /|
//     (1) +Z |
//           -Y (5)
//

impl Iterator for NeighborsPosIterator {
    type Item = Voxel;

    fn next(&mut self) -> Option<Self::Item> {
        let Voxel(x, y, z) = self.voxel;
        match self.face_idx {
            6 => None,
            _ => {
                let neighbor = match self.face_idx {
                    0 => Voxel(x + 1, y, z),
                    1 => Voxel(x, y, z + 1),
                    2 => Voxel(x - 1, y, z),
                    3 => Voxel(x, y, z - 1),
                    4 => Voxel(x, y + 1, z),
                    5 => Voxel(x, y - 1, z),
                    _ => unreachable!(),
                };
                self.face_idx += 1;
                Some(neighbor)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Extent(Voxel, Voxel);

impl Extent {
    fn includes(&self, voxel: &Voxel) -> bool {
        let Voxel(x0, y0, z0) = self.0;
        let Voxel(x1, y1, z1) = self.1;
        let &Voxel(x, y, z) = voxel;

        x0 <= x && y0 <= y && z0 <= z && x1 >= x && y1 >= y && z1 >= z
    }
}

impl From<Vec<&Voxel>> for Extent {
    fn from(voxels: Vec<&Voxel>) -> Self {
        let mut iter_x = voxels.iter().map(|v| v.0).sorted().peekable();
        let mut iter_y = voxels.iter().map(|v| v.1).sorted().peekable();
        let mut iter_z = voxels.iter().map(|v| v.2).sorted().peekable();

        let x0 = iter_x.peek().copied().unwrap();
        let x1 = iter_x.last().unwrap();
        let y0 = iter_y.peek().copied().unwrap();
        let y1 = iter_y.last().unwrap();
        let z0 = iter_z.peek().copied().unwrap();
        let z1 = iter_z.last().unwrap();

        Self(Voxel(x0, y0, z0), Voxel(x1, y1, z1))
    }
}

fn parse_voxels(input: &str) -> HashSet<Voxel> {
    input.trimmed_lines().map(|s| s.into()).collect()
}

pub fn count_faces(voxels: &HashSet<Voxel>) -> u64 {
    voxels
        .iter()
        .map(|voxel| {
            voxel
                .neighbors()
                .map(|pos| match voxels.get(&pos) {
                    Some(_) => 0,
                    None => 1,
                })
                .sum::<u64>()
        })
        .sum()
}

pub fn part01(input: &str) -> u64 {
    count_faces(&parse_voxels(input))
}

fn find_connected_voxels(
    start_voxel: &Voxel,
    extent: Extent,
    voxels: &HashSet<Voxel>,
) -> HashSet<Voxel> {
    fn find_connected_voxels_rec(
        current_voxel: &Voxel,
        extent: Extent,
        voxels: &HashSet<Voxel>,
        collected: &mut HashSet<Voxel>,
    ) {
        // return early if the current voxel is not part of the given voxels
        if voxels.contains(current_voxel) {
            return;
        }
        // add the current voxel to the collected voxels
        collected.insert(*current_voxel);
        // return early if the current voxel is outside the given extent
        if !extent.includes(current_voxel) {
            return;
        }
        // recursively find all connected voxels
        for neighbor in current_voxel.neighbors() {
            // skip the neighbor if it has already been collected or is not part of the given voxels
            if collected.contains(&neighbor) || voxels.contains(&neighbor) {
                continue;
            }
            find_connected_voxels_rec(&neighbor, extent, voxels, collected);
        }
    }
    let mut collected = HashSet::<Voxel>::new();
    find_connected_voxels_rec(start_voxel, extent, voxels, &mut collected);
    collected
}


fn hole_at(voxel: &Voxel, extent: Extent, voxels: &HashSet<Voxel>) -> Option<HashSet<Voxel>> {
    let con = find_connected_voxels(voxel, extent, voxels);
    let over_extent = con.iter().any(|v| !extent.includes(v));
    let is_hole = !con.is_empty() && !over_extent;

    match is_hole {
        true => Some(con),
        false => None,
    }
}

pub fn part02(input: &str) -> u64 {
    let mut voxels = parse_voxels(input);

    // calculate boundaries
    let extent = Extent::from(Vec::from_iter(voxels.iter()));

    // collect potential holes
    let maybe_holes = voxels
        .iter()
        .fold(HashSet::<Voxel>::new(), |mut accu, voxel| {
            voxel.neighbors().for_each(|v| {
                if !voxels.contains(&v) {
                    accu.insert(v);
                }
            });
            accu
        });

    // fill up actual holes
    for maybe_hole in maybe_holes.iter() {
        if let Some(hole) = hole_at(maybe_hole, extent, &voxels) {
            voxels.extend(hole);
        }
    }

    // return face count
    count_faces(&voxels)
}

#[cfg(test)]
mod tests {
    use crate::{
        day18::{part01, part02},
        input,
    };

    const SMALL_EXAMPLE: &str = "\
    1,1,1
    2,1,1";

    const SMALL_EXAMPLE_WITH_HOLE: &str = "\
    0,-1,0
    1,0,0
    0,0,-1
    -1,0,0
    0,0,1
    0,1,0";

    const LARGER_EXAMPLE_WITH_HOLE: &str = "\
    2,2,2
    1,2,2
    3,2,2
    2,1,2
    2,3,2
    2,2,1
    2,2,3
    2,2,4
    2,2,6
    1,2,5
    3,2,5
    2,1,5
    2,3,5";

    #[test]
    fn day_18_part_01_small_example() {
        assert_eq!(part01(SMALL_EXAMPLE), 10);
    }

    #[test]
    fn day_18_part_01_small_example_with_hole() {
        assert_eq!(part01(SMALL_EXAMPLE_WITH_HOLE), 36);
    }

    #[test]
    fn day_18_part_01_larger_example_with_hole() {
        assert_eq!(part01(LARGER_EXAMPLE_WITH_HOLE), 64);
    }

    #[test]
    fn day_18_part_01() {
        assert_eq!(part01(input("day_18").as_str()), 4364);
    }

    #[test]
    fn day_18_part_02_small_example() {
        assert_eq!(part02(SMALL_EXAMPLE), 10);
    }

    #[test]
    fn day_18_part_02_small_example_with_hole() {
        assert_eq!(part02(SMALL_EXAMPLE_WITH_HOLE), 30);
    }

    #[test]
    fn day_18_part_02_larger_example_with_hole() {
        assert_eq!(part02(LARGER_EXAMPLE_WITH_HOLE), 58);
    }

    #[test]
    fn day_18_larger_example_part2_3() {
        assert_eq!(part02(input("day_18").as_str()), 2508);
    }
}
