use std::collections::VecDeque;

use factorial::Factorial;
use fnv::{FnvHashMap, FnvHashSet, FnvHasher};
use regex::Regex;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| line.parse::<u64>().expect("Error while parsing input"))
        .collect()
}

#[aoc(day10, part1, naive)]
pub fn solve_part1_naive(input: &[u64]) -> u64 {
    let mut adapters = input.to_vec();
    // We can't know what the lowest number is, and we always start at 0
    let mut ones = 0;
    // We do know that our device always has a built in 3, so just count it now
    let mut threes = 1;
    // Sort all our adapters
    adapters.sort_unstable();

    for x in 0..adapters.len() {
        // We can't index -1 for 0, but we know we start at the value zero, so just use that
        if x == 0 {
            if adapters[x] - 0 == 1 {
                ones += 1;
            } else if adapters[x] - 0 == 3 {
                threes += 1;
            }
        }
        // Everywhere else, we just use the index and index-1 to see if we have a difference of one or three
        else {
            if adapters[x] - adapters[x - 1] == 1 {
                ones += 1;
            } else if adapters[x] - adapters[x - 1] == 3 {
                threes += 1;
            }
        }
    }

    ones * threes
}

#[aoc(day10, part1, iter)]
pub fn solve_part1_iter(input: &[u64]) -> u64 {
    let mut adapters = input.to_vec();
    adapters.push(0);
    // Sort all our adapters
    adapters.sort_unstable();
    adapters.push(adapters.last().unwrap() + 3);

    let mut diffs = [0; 3];

    adapters
        .iter()
        .zip(adapters.iter().skip(1))
        .for_each(|(a, b)| diffs[(*b - *a - 1) as usize] += 1);

    diffs[0] * diffs[2]
}

pub fn count_chains_cached(
    adapters: &[u64],
    index: usize,
    cache: &mut FnvHashMap<usize, u64>,
) -> u64 {
    if index >= adapters.len() - 1 {
        1
    } else if let Some(cache_hit) = cache.get(&index) {
        *cache_hit
    } else {
        let val = adapters
            .iter()
            .skip(index + 1)
            .take(3)
            .enumerate()
            .filter(|(_, val)| **val - adapters[index] <= 3)
            .map(|(idx, _)| count_chains_cached(adapters, index + idx + 1, cache))
            .sum();

        cache.insert(index, val);

        val
    }
}

#[aoc(day10, part2, cache)]
pub fn solve_part2_cache(input: &[u64]) -> u64 {
    let mut adapters = input.to_vec();
    adapters.push(0);
    // Sort all our adapters
    adapters.sort_unstable();
    adapters.push(adapters.last().unwrap() + 3);

    let mut cache = FnvHashMap::default();

    count_chains_cached(&adapters, 0, &mut cache)
}

#[aoc(day10, part2, mask)]
pub fn solve_part2_mask(input: &[u64]) -> u64 {
    let mut adapters = input.to_vec();
    adapters.push(0);
    // Sort all our adapters
    adapters.sort_unstable();

    let max = *adapters.last().unwrap();

    adapters.push(adapters.last().unwrap() + 3);

    let mut mask = vec![0; *adapters.last().unwrap() as usize + 1];
    for i in adapters {
        mask[i as usize] = 1;
    }

    (0..max).rev().for_each(|v| {
        if mask[v as usize] > 0 {
            mask[v as usize] = mask[v as usize + 1] + mask[v as usize + 2] + mask[v as usize + 3];
        }
    });

    mask[0]
}
