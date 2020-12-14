use std::collections::VecDeque;

use factorial::Factorial;
use fnv::{FnvHashMap, FnvHashSet, FnvHasher};
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl Seat {
    fn from_char(input: &char) -> Seat {
        match input {
            '.' => Seat::Floor,
            'L' => Seat::Empty,
            '#' => Seat::Occupied,
            _ => unreachable!("Unhandled seat layout character"),
        }
    }

    fn is_occupied(&self) -> bool {
        match self {
            Self::Occupied => true,
            _ => false,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Slope(i32, i32);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coords(u32, u32);

#[derive(Debug, Clone)]
pub struct Boat {
    seats: FnvHashMap<Coords, Seat>,
    neighborhood_map: FnvHashMap<Coords, Vec<Coords>>,
    visible_neighborhood_map: FnvHashMap<Coords, Vec<Coords>>,
    update_map: FnvHashMap<Coords, bool>,
}

impl Boat {
    fn update_seat(&mut self, coords: &Coords) {
        let occupied_neighbors = self.count_occupied_neighbors(coords);

        if occupied_neighbors == 0
            && self
                .seats
                .get(coords)
                .expect("Unable to get our own seat?")
                .is_empty()
        {
            *self
                .update_map
                .get_mut(coords)
                .expect("Unable to retrieve update map entry") = true
        } else if occupied_neighbors >= 4
            && self
                .seats
                .get(coords)
                .expect("Unable to get our own seat?")
                .is_occupied()
        {
            *self
                .update_map
                .get_mut(coords)
                .expect("Unable to retrieve update map entry") = true
        } else {
        }
    }

    fn update_seat_part2(&mut self, coords: &Coords) {
        let occupied_neighbors = self.count_occupied_neighbors_part2(coords);

        if occupied_neighbors == 0
            && self
                .seats
                .get(coords)
                .expect("Unable to get our own seat?")
                .is_empty()
        {
            *self
                .update_map
                .get_mut(coords)
                .expect("Unable to retrieve update map entry") = true
        } else if occupied_neighbors >= 5
            && self
                .seats
                .get(coords)
                .expect("Unable to get our own seat?")
                .is_occupied()
        {
            *self
                .update_map
                .get_mut(coords)
                .expect("Unable to retrieve update map entry") = true
        } else {
        }
    }

    fn count_occupied_neighbors(&self, coords: &Coords) -> usize {
        if let Some(neighbors) = self.neighborhood_map.get(coords) {
            neighbors
                .iter()
                .map(|neighbor| {
                    self.seats
                        .get(neighbor)
                        .expect("Unable to retrieve seat for neighbor!")
                })
                .filter(|seat| seat.is_occupied())
                .count()
        } else {
            panic!("Failed to retrieve a neighbor map, even empty!")
        }
    }

    fn count_occupied_neighbors_part2(&self, coords: &Coords) -> usize {
        if let Some(neighbors) = self.visible_neighborhood_map.get(coords) {
            neighbors
                .iter()
                .map(|neighbor| {
                    self.seats
                        .get(neighbor)
                        .expect("Unable to retrieve seat for neighbor!")
                })
                .filter(|seat| seat.is_occupied())
                .count()
        } else {
            panic!("Failed to retrieve a neighbor map, even empty!")
        }
    }

    fn apply_update_map(&mut self) {
        for (coords, val) in &mut self.update_map {
            if *val == true {
                let seat = self
                    .seats
                    .get_mut(coords)
                    .expect("Unable to get seat from update map coords");
                match seat {
                    Seat::Empty => *seat = Seat::Occupied,
                    Seat::Occupied => *seat = Seat::Empty,
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

    fn count_occupied_seats(&self) -> usize {
        let mut seats = 0;

        for (_, seat) in &self.seats {
            if *seat == Seat::Occupied {
                seats += 1;
            }
        }

        seats
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Boat {
    let mut seats = FnvHashMap::default();
    let mut neighborhood_map = FnvHashMap::default();
    let mut visible_neighborhood_map = FnvHashMap::default();
    let mut update_map = FnvHashMap::default();

    input.lines().enumerate().for_each(|(idx, line)| {
        let mut x_coord = 0;

        line.chars().for_each(|c| {
            seats.insert(Coords(x_coord, idx as u32 - 1), Seat::from_char(&c));
            update_map.insert(Coords(x_coord, idx as u32 - 1), false);
            x_coord += 1;
        });
    });

    seats.iter().for_each(|(coords, _)| {
        let mut neighbor_vec = Vec::new();

        let nw = Coords(coords.0 - 1, coords.1 - 1);
        let n = Coords(coords.0, coords.1 - 1);
        let ne = Coords(coords.0 + 1, coords.1 - 1);
        let e = Coords(coords.0 + 1, coords.1);
        let se = Coords(coords.0 + 1, coords.1 + 1);
        let s = Coords(coords.0, coords.1 + 1);
        let sw = Coords(coords.0 - 1, coords.1 + 1);
        let w = Coords(coords.0 - 1, coords.1);

        for ea in [nw, n, ne, e, se, s, sw, w].iter() {
            if let Some(_) = seats.get(&ea) {
                neighbor_vec.push(*ea);
            }
        }

        neighborhood_map.insert(*coords, neighbor_vec);
    });

    seats.iter().for_each(|(coords, _)| {
        let mut neighbor_vec = Vec::new();

        let nw = Slope(-1, -1);
        let n = Slope(0, -1);
        let ne = Slope(1, -1);
        let e = Slope(1, 0);
        let se = Slope(1, 1);
        let s = Slope(0, 1);
        let sw = Slope(-1, 1);
        let w = Slope(-1, 0);

        for ea in [nw, n, ne, e, se, s, sw, w].iter() {
            let mut idx = 1;
            loop {
                let view_coords = Coords(
                    coords.0.wrapping_add(idx * ea.0 as u32),
                    coords.1.wrapping_add(idx * ea.1 as u32),
                );
                if let Some(seat) = seats.get(&view_coords) {
                    match seat {
                        Seat::Floor => idx += 1,
                        _ => {
                            neighbor_vec.push(view_coords);
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        }

        visible_neighborhood_map.insert(*coords, neighbor_vec);
    });

    Boat {
        seats,
        neighborhood_map,
        visible_neighborhood_map,
        update_map,
    }
}

#[aoc(day11, part1, naive)]
pub fn solve_part1_naive(input: &Boat) -> usize {
    let mut boat = input.clone();

    loop {
        for coord in input.seats.keys() {
            boat.update_seat(coord);
        }

        if boat.count_changes() == 0 {
            return boat.count_occupied_seats();
        }

        boat.apply_update_map();
    }
}

#[aoc(day11, part2, naive)]
pub fn solve_part2_naive(input: &Boat) -> usize {
    let mut boat = input.clone();
    let mut iterations = 0;
    loop {
        for coord in input.seats.keys() {
            boat.update_seat_part2(coord);
        }

        if boat.count_changes() == 0 {
            return boat.count_occupied_seats();
        }

        boat.apply_update_map();
    }
}
