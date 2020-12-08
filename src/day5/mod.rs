use fnv::FnvHashMap;
use regex::Regex;
// use std::collections::HashMap;

lazy_static! {
    static ref BOARDING_PASS: Regex = Regex::new(r"([FB]{7})([LR]{3})").unwrap();
}

/// Holds all Boarding pass fields
#[derive(Debug, Clone, Default)]
pub struct BoardingPass {
    /// Seat Row
    row: usize,
    /// Seat Column
    column: usize,
}

impl BoardingPass {
    pub fn from_str(input: &str) -> Self {
        let boarding_pass = BOARDING_PASS
            .captures(input)
            .expect("Error parsing boarding pass data");

        let row = boarding_pass
            .get(1)
            .expect("Error getting row capture value")
            .as_str()
            .chars()
            .enumerate()
            .map(|(idx, val)| match val {
                'B' => 1 << (6 - (idx)),
                'F' => 0,
                _ => unreachable!(),
            })
            .sum();

        let column = boarding_pass
            .get(2)
            .expect("Error getting column capture value")
            .as_str()
            .chars()
            .enumerate()
            .map(|(idx, val)| match val {
                'R' => 1 << (2 - (idx)),
                'L' => 0,
                _ => unreachable!(),
            })
            .sum();

        BoardingPass { row, column }
    }
}

impl BoardingPass {
    /// Simple Unique seat ID generation based on binary conversion
    pub fn get_seat_id(&self) -> usize {
        self.row * 8 + self.column
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<BoardingPass> {
    let vec: Vec<BoardingPass> = input
        .lines()
        .map(|bpdata| BoardingPass::from_str(bpdata))
        .collect();

    vec
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[BoardingPass]) -> usize {
    input
        .iter()
        .map(BoardingPass::get_seat_id)
        .max()
        .expect("Error getting max value")
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[BoardingPass]) -> usize {
    let mut seat_map = FnvHashMap::default();
    for bp in input {
        seat_map.insert(bp.get_seat_id(), bp);
    }

    seat_map
        .iter()
        // We only have a list of everyone else's boarding passes, so we don't know where the hole is
        // we need to modify the requirements to a - 2/-1 instead of a +1/-1 that way we know we can find the seat
        // with the data we actually have
        .find(|(key, _)| {
            seat_map.get(&(*key - 2)).is_some() && seat_map.get(&(*key - 1)).is_none()
        })
        // At this point we should have the key-value pair that we need
        .expect("Could not find a seat matching the requirements")
        // We get the key, which is the seat id
        .0
        // Then we subtract one, because of how we did the search, we actually searched off of the seat +1 to ours
        - 1
}
