use std::collections::VecDeque;

use factorial::Factorial;
use fnv::{FnvHashMap, FnvHashSet, FnvHasher};
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConwayCube {
    Active,
    Inactive,
}

impl ConwayCube {
    fn from_char(input: &char) -> Self {
        match input {
            '.' => Self::Inactive,
            '#' => Self::Active,
            _ => unreachable!("Unhandled ConwayCube activity character"),
        }
    }

    fn is_active(&self) -> bool {
        match self {
            Self::Active => true,
            _ => false,
        }
    }

    fn is_inactive(&self) -> bool {
        match self {
            Self::Inactive => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
// pub struct Coords(i32, i32, i32);
pub struct Coords(i32, i32, i32, i32);

#[derive(Debug, Clone)]
pub struct PocketDimension {
    cubes: FnvHashMap<Coords, ConwayCube>,
    // neighborhood_map: FnvHashMap<Coords, [Coords; 26]>,
    neighborhood_map: FnvHashMap<Coords, [Coords; 80]>,
    update_map: FnvHashMap<Coords, bool>,
}

impl PocketDimension {
    fn update_cube(&mut self, coords: &Coords) {
        let active_neighbors = self.count_active_neighbors(coords);

        if active_neighbors == 3
            && self
                .cubes
                .get(coords)
                .expect("Unable to get our own seat?")
                .is_inactive()
        {
            *self
                .update_map
                .get_mut(coords)
                .expect("Could not get Updatemap entry") = true
        } else if (active_neighbors != 2 && active_neighbors != 3)
            && self
                .cubes
                .get(coords)
                .expect("Unable to get our own seat?")
                .is_active()
        {
            *self
                .update_map
                .get_mut(coords)
                .expect("Could not get Updatemap entry") = true
        } else {
            // Nothing happens
        }
    }

    fn populate_all_neighbors(&mut self) {
        let mut coords: FnvHashSet<Coords> = FnvHashSet::default();
        self.neighborhood_map.iter().for_each(|(_, list)| {
            list.iter().for_each(|x| {
                coords.insert(*x);
            });
        });

        for coord in coords {
            if let Some(neighbors) = self.neighborhood_map.get(&coord) {
                for neighbor in neighbors {
                    self.cubes.entry(*neighbor).or_insert(ConwayCube::Inactive);
                }
            }
            self.update_map.insert(coord, false);
            // self.neighborhood_map.insert(coord, get_neighbors_3d(coord));
            self.neighborhood_map.insert(coord, get_neighbors_4d(coord));
        }
    }

    fn count_active_neighbors(&self, coords: &Coords) -> usize {
        if let Some(neighbors) = self.neighborhood_map.get(coords) {
            neighbors
                .iter()
                .map(|neighbor| {
                    if let Some(cube) = self.cubes.get(neighbor) {
                        *cube
                    } else {
                        ConwayCube::Inactive
                    }
                })
                .filter(|cube| cube.is_active())
                .count()
        } else {
            panic!("Failed to retrieve a neighbor map, even empty!")
        }
    }

    fn apply_update_map(&mut self) {
        for (coords, val) in &mut self.update_map {
            if *val == true {
                let cube = self
                    .cubes
                    .get_mut(coords)
                    .expect("Unable to get cube from update map coords");
                match cube {
                    ConwayCube::Inactive => *cube = ConwayCube::Active,
                    ConwayCube::Active => *cube = ConwayCube::Inactive,
                    _ => unreachable!(),
                }

                *val = false;
            }
        }
    }

    fn count_changes(&self) -> usize {
        let mut changes = 0;
        for (_, val) in &self.update_map {
            if *val == true {
                changes += 1;
            }
        }

        changes
    }

    fn count_active_cubes(&self) -> usize {
        let mut cubes = 0;

        for (_, cube) in &self.cubes {
            if *cube == ConwayCube::Active {
                cubes += 1;
            }
        }

        cubes
    }
}

// pub fn get_neighbors_3d(input: Coords) -> [Coords; 26] {
//     [
//         Coords(input.0 - 1, input.1 - 1, input.2 - 1),
//         Coords(input.0 - 1, input.1 - 1, input.2),
//         Coords(input.0 - 1, input.1 - 1, input.2 + 1),
//         Coords(input.0, input.1 - 1, input.2 - 1),
//         Coords(input.0, input.1 - 1, input.2),
//         Coords(input.0, input.1 - 1, input.2 + 1),
//         Coords(input.0 + 1, input.1 - 1, input.2 - 1),
//         Coords(input.0 + 1, input.1 - 1, input.2),
//         Coords(input.0 + 1, input.1 - 1, input.2 + 1),
//         Coords(input.0 - 1, input.1, input.2 - 1),
//         Coords(input.0 - 1, input.1, input.2),
//         Coords(input.0 - 1, input.1, input.2 + 1),
//         Coords(input.0, input.1, input.2 - 1),
//         Coords(input.0, input.1, input.2 + 1),
//         Coords(input.0 + 1, input.1, input.2 - 1),
//         Coords(input.0 + 1, input.1, input.2),
//         Coords(input.0 + 1, input.1, input.2 + 1),
//         Coords(input.0 - 1, input.1 + 1, input.2 - 1),
//         Coords(input.0 - 1, input.1 + 1, input.2),
//         Coords(input.0 - 1, input.1 + 1, input.2 + 1),
//         Coords(input.0, input.1 + 1, input.2 - 1),
//         Coords(input.0, input.1 + 1, input.2),
//         Coords(input.0, input.1 + 1, input.2 + 1),
//         Coords(input.0 + 1, input.1 + 1, input.2 - 1),
//         Coords(input.0 + 1, input.1 + 1, input.2),
//         Coords(input.0 + 1, input.1 + 1, input.2 + 1),
//     ]
// }

pub fn get_neighbors_4d(input: Coords) -> [Coords; 80] {
    [
        Coords(input.0 - 1, input.1 - 1, input.2 - 1, input.3 - 1),
        Coords(input.0 - 1, input.1 - 1, input.2, input.3 - 1),
        Coords(input.0 - 1, input.1 - 1, input.2 + 1, input.3 - 1),
        Coords(input.0, input.1 - 1, input.2 - 1, input.3 - 1),
        Coords(input.0, input.1 - 1, input.2, input.3 - 1),
        Coords(input.0, input.1 - 1, input.2 + 1, input.3 - 1),
        Coords(input.0 + 1, input.1 - 1, input.2 - 1, input.3 - 1),
        Coords(input.0 + 1, input.1 - 1, input.2, input.3 - 1),
        Coords(input.0 + 1, input.1 - 1, input.2 + 1, input.3 - 1),
        Coords(input.0 - 1, input.1, input.2 - 1, input.3 - 1),
        Coords(input.0 - 1, input.1, input.2, input.3 - 1),
        Coords(input.0 - 1, input.1, input.2 + 1, input.3 - 1),
        Coords(input.0, input.1, input.2 - 1, input.3 - 1),
        Coords(input.0, input.1, input.2, input.3 - 1),
        Coords(input.0, input.1, input.2 + 1, input.3 - 1),
        Coords(input.0 + 1, input.1, input.2 - 1, input.3 - 1),
        Coords(input.0 + 1, input.1, input.2, input.3 - 1),
        Coords(input.0 + 1, input.1, input.2 + 1, input.3 - 1),
        Coords(input.0 - 1, input.1 + 1, input.2 - 1, input.3 - 1),
        Coords(input.0 - 1, input.1 + 1, input.2, input.3 - 1),
        Coords(input.0 - 1, input.1 + 1, input.2 + 1, input.3 - 1),
        Coords(input.0, input.1 + 1, input.2 - 1, input.3 - 1),
        Coords(input.0, input.1 + 1, input.2, input.3 - 1),
        Coords(input.0, input.1 + 1, input.2 + 1, input.3 - 1),
        Coords(input.0 + 1, input.1 + 1, input.2 - 1, input.3 - 1),
        Coords(input.0 + 1, input.1 + 1, input.2, input.3 - 1),
        Coords(input.0 + 1, input.1 + 1, input.2 + 1, input.3 - 1),

        Coords(input.0 - 1, input.1 - 1, input.2 - 1, input.3),
        Coords(input.0 - 1, input.1 - 1, input.2, input.3),
        Coords(input.0 - 1, input.1 - 1, input.2 + 1, input.3),
        Coords(input.0, input.1 - 1, input.2 - 1, input.3),
        Coords(input.0, input.1 - 1, input.2, input.3),
        Coords(input.0, input.1 - 1, input.2 + 1, input.3),
        Coords(input.0 + 1, input.1 - 1, input.2 - 1, input.3),
        Coords(input.0 + 1, input.1 - 1, input.2, input.3),
        Coords(input.0 + 1, input.1 - 1, input.2 + 1, input.3),
        Coords(input.0 - 1, input.1, input.2 - 1, input.3),
        Coords(input.0 - 1, input.1, input.2, input.3),
        Coords(input.0 - 1, input.1, input.2 + 1, input.3),
        Coords(input.0, input.1, input.2 - 1, input.3),
        Coords(input.0, input.1, input.2 + 1, input.3),
        Coords(input.0 + 1, input.1, input.2 - 1, input.3),
        Coords(input.0 + 1, input.1, input.2, input.3),
        Coords(input.0 + 1, input.1, input.2 + 1, input.3),
        Coords(input.0 - 1, input.1 + 1, input.2 - 1, input.3),
        Coords(input.0 - 1, input.1 + 1, input.2, input.3),
        Coords(input.0 - 1, input.1 + 1, input.2 + 1, input.3),
        Coords(input.0, input.1 + 1, input.2 - 1, input.3),
        Coords(input.0, input.1 + 1, input.2, input.3),
        Coords(input.0, input.1 + 1, input.2 + 1, input.3),
        Coords(input.0 + 1, input.1 + 1, input.2 - 1, input.3),
        Coords(input.0 + 1, input.1 + 1, input.2, input.3),
        Coords(input.0 + 1, input.1 + 1, input.2 + 1, input.3),

        Coords(input.0 - 1, input.1 - 1, input.2 - 1, input.3 + 1),
        Coords(input.0 - 1, input.1 - 1, input.2, input.3 + 1),
        Coords(input.0 - 1, input.1 - 1, input.2 + 1, input.3 + 1),
        Coords(input.0, input.1 - 1, input.2 - 1, input.3 + 1),
        Coords(input.0, input.1 - 1, input.2, input.3 + 1),
        Coords(input.0, input.1 - 1, input.2 + 1, input.3 + 1),
        Coords(input.0 + 1, input.1 - 1, input.2 - 1, input.3 + 1),
        Coords(input.0 + 1, input.1 - 1, input.2, input.3 + 1),
        Coords(input.0 + 1, input.1 - 1, input.2 + 1, input.3 + 1),
        Coords(input.0 - 1, input.1, input.2 - 1, input.3 + 1),
        Coords(input.0 - 1, input.1, input.2, input.3 + 1),
        Coords(input.0 - 1, input.1, input.2 + 1, input.3 + 1),
        Coords(input.0, input.1, input.2 - 1, input.3 + 1),
        Coords(input.0, input.1, input.2, input.3 + 1),
        Coords(input.0, input.1, input.2 + 1, input.3 + 1),
        Coords(input.0 + 1, input.1, input.2 - 1, input.3 + 1),
        Coords(input.0 + 1, input.1, input.2, input.3 + 1),
        Coords(input.0 + 1, input.1, input.2 + 1, input.3 + 1),
        Coords(input.0 - 1, input.1 + 1, input.2 - 1, input.3 + 1),
        Coords(input.0 - 1, input.1 + 1, input.2, input.3 + 1),
        Coords(input.0 - 1, input.1 + 1, input.2 + 1, input.3 + 1),
        Coords(input.0, input.1 + 1, input.2 - 1, input.3 + 1),
        Coords(input.0, input.1 + 1, input.2, input.3 + 1),
        Coords(input.0, input.1 + 1, input.2 + 1, input.3 + 1),
        Coords(input.0 + 1, input.1 + 1, input.2 - 1, input.3 + 1),
        Coords(input.0 + 1, input.1 + 1, input.2, input.3 + 1),
        Coords(input.0 + 1, input.1 + 1, input.2 + 1, input.3 + 1),
    ]
}

// #[aoc_generator(day17, part1)]
// pub fn input_generator(input: &str) -> PocketDimension {
//     let mut cubes = FnvHashMap::default();
//     let mut neighborhood_map = FnvHashMap::default();
//     let mut update_map = FnvHashMap::default();

//     input.lines().enumerate().for_each(|(idx, line)| {
//         line.chars().enumerate().for_each(|(x_idx, c)| {
//             cubes.insert(
//                 Coords(x_idx as i32, idx as i32, 0),
//                 ConwayCube::from_char(&c),
//             );
//             update_map.insert(Coords(x_idx as i32, idx as i32, 0), false);
//         });
//     });

//     cubes.iter().for_each(|(coords, _)| {
//         let neighbors = get_neighbors_3d(*coords);
//         neighborhood_map.insert(*coords, neighbors);
//     });

//     PocketDimension {
//         cubes,
//         neighborhood_map,
//         update_map,
//     }
// }

#[aoc_generator(day17, part2)]
pub fn input_generator_p2(input: &str) -> PocketDimension {
    let mut cubes = FnvHashMap::default();
    let mut neighborhood_map = FnvHashMap::default();
    let mut update_map = FnvHashMap::default();

    input.lines().enumerate().for_each(|(idx, line)| {
        line.chars().enumerate().for_each(|(x_idx, c)| {
            cubes.insert(
                Coords(x_idx as i32, idx as i32, 0, 0),
                ConwayCube::from_char(&c),
            );
            update_map.insert(Coords(x_idx as i32, idx as i32, 0, 0), false);
        });
    });

    cubes.iter().for_each(|(coords, _)| {
        let neighbors = get_neighbors_4d(*coords);
        neighborhood_map.insert(*coords, neighbors);
    });

    PocketDimension {
        cubes,
        neighborhood_map,
        update_map,
    }
}

// #[aoc(day17, part1, naive)]
// pub fn solve_part1_naive(input: &PocketDimension) -> usize {
//     let mut pd = input.clone();

//     for _ in 0..6 {
//         pd.populate_all_neighbors();

//         let coords: Vec<Coords> = pd.cubes.keys().map(|x| *x).collect();
//         for coord in coords {
//             pd.update_cube(&coord);
//         }

//         pd.apply_update_map();
//     }
//     return pd.count_active_cubes();
// }

#[aoc(day17, part2, naive)]
pub fn solve_part2_naive(input: &PocketDimension) -> usize {
    let mut pd = input.clone();

    for _ in 0..6 {
        pd.populate_all_neighbors();

        let coords: Vec<Coords> = pd.cubes.keys().map(|x| *x).collect();
        for coord in coords {
            pd.update_cube(&coord);
        }

        pd.apply_update_map();
    }
    return pd.count_active_cubes();
}
