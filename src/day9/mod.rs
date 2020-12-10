use std::collections::VecDeque;

use factorial::Factorial;
use fnv::{FnvHashMap, FnvHashSet};

static WINDOW_SIZE: usize = 25;
static mut PART_1: u64 = 0;

pub fn calc_total_combinations(n: u128, r: u128) -> usize {
    (n.factorial() / (r.factorial() * (n - r).factorial())) as usize
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| line.parse::<u64>().expect("Error while parsing input"))
        .collect()
}

pub fn populate_vecdeque(v: &mut VecDeque<u64>, window: &[u64]) {
    v.clear();
    for x in 0..window.len() {
        for y in x + 1..window.len() {
            v.push_back(window[x] + window[y]);
        }
    }
}

#[aoc(day9, part1, vecdeque)]
pub fn solve_part1_vecdeque(input: &[u64]) -> u64 {
    // Pre-calculate the size of our VecDeque
    let total_combinations = calc_total_combinations(WINDOW_SIZE as u128, 2);
    // Initialize our VecDeque
    let mut values: VecDeque<u64> = VecDeque::with_capacity(total_combinations);

    // Build our window into the input
    let mut index = 0;

    loop {
        let window = &input[index..index + WINDOW_SIZE];

        populate_vecdeque(&mut values, window);

        if !values.contains(&input[WINDOW_SIZE + index]) {
            unsafe {
                PART_1 = input[WINDOW_SIZE + index];
            }
            return input[WINDOW_SIZE + index];
        }

        index += 1;
    }
}

pub fn populate_vec(v: &mut Vec<u64>, window: &[u64]) {
    v.clear();
    for x in 0..window.len() {
        for y in x + 1..window.len() {
            v.push(window[x] + window[y]);
        }
    }
}

#[aoc(day9, part1, Vec)]
pub fn solve_part1_vec(input: &[u64]) -> u64 {
    // Pre-calculate the size of our VecDeque
    let total_combinations = calc_total_combinations(WINDOW_SIZE as u128, 2);
    // Initialize our VecDeque
    let mut values: Vec<u64> = Vec::with_capacity(total_combinations);

    // Build our window into the input
    let mut index = 0;

    loop {
        let window = &input[index..index + WINDOW_SIZE];

        populate_vec(&mut values, window);

        if !values.contains(&input[WINDOW_SIZE + index]) {
            return input[WINDOW_SIZE + index];
        }

        index += 1;
    }
}

#[aoc(day9, part1, fnvhashset)]
pub fn solve_part1_fnvhashset(input: &[u64]) -> u64 {
    let mut values: FnvHashSet<(u64, u64)> = FnvHashSet::default();

    // Build our window into the input
    let mut index = 0;

    // Initialize our Set  by inserting all the (Value, Parent) pairs
    let window = &input[index..index + WINDOW_SIZE];
    for x in 0..window.len() {
        for y in x + 1..window.len() {
            match values.insert((window[x] + window[y], window[x])) {
                true => {}
                false => panic!("Duplicate key"),
            }
        }
    }

    let mut keys: Vec<(u64, u64)> = Vec::with_capacity(WINDOW_SIZE);
    loop {
        // If we can't find a match in our values Set, then that is the value we need to return
        if values
            .iter()
            .find(|(val, _)| *val == input[WINDOW_SIZE + index])
            .is_none()
        {
            return input[WINDOW_SIZE + index];
        }

        // Purge the set of values related to the beginning of the window before we move it
        &keys.clear();
        for (key, val) in &values {
            if *val == input[index] {
                // Record all the keys that are related to the window start value
                &keys.push((*key, *val));
            }
        }
        for key in &keys {
            // Remove all the keys that are related to the window start value
            values.remove(&key);
        }

        // Populate the set with new values related to the next value outside the window
        for idx in 0..WINDOW_SIZE - 1 {
            values.insert((
                input[index + idx + 1] + input[index + WINDOW_SIZE],
                input[index + WINDOW_SIZE],
            ));
        }

        index += 1;
    }
}

#[aoc(day9, part2, naive)]
pub fn solve_part2_naive(input: &[u64]) -> u64 {
    let target = unsafe { PART_1 };
    // Start the loop from 0
    let mut start_index = 0;
    // This loop essentially will go through `input` and just sum from `start_index` until the end
    // if `sum_total` is ever > target, we know that can't be the right window and move on to the
    // next starting index
    loop {
        // Start our sum from zero for each index
        let mut sum_total = 0;
        for x in start_index..input.len() {
            // Sum each input until we find the target, or overshoot it
            sum_total += input[x];

            // If we find the target, use iterators to get our real return value
            if sum_total == target {
                return input[start_index..x].iter().min().unwrap()
                    + input[start_index..x].iter().max().unwrap();
            }
            // If we overshoot our target, just move on to the next starting index
            else if sum_total > target {
                break;
            }
        }
        start_index += 1;
    }
}

#[aoc(day9, part2, inchworm)]
pub fn solve_part2_inchworm(input: &[u64]) -> u64 {
    let target = unsafe { PART_1 };
    let mut start_index = 0;
    let mut end_index = 0;
    let mut sum_total = 0;

    loop {
        // Sum contiguous values until we overshoot or find it
        while sum_total < target {
            sum_total += input[end_index];
            end_index += 1;
        }

        // If we found it, just return our answer with some iterator magic
        if sum_total == target {
            return input[start_index..end_index].iter().min().unwrap()
                + input[start_index..end_index].iter().max().unwrap();
        }
        // If we overshot, we want to subtract the numbers from the start of the list until we are back under and
        // then we try again, this way we will inch our way towards whatever is the actual range. We can leverage
        // this because the numbers have to be contiguous
        else if sum_total > target {
            while sum_total > target {
                sum_total -= input[start_index];
                start_index += 1;
            }
        }
    }
}
