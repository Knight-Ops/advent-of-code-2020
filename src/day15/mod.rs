use fnv::{FnvHashMap, FnvHashSet};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct MemoryGame {
    turn: usize,
    memory: FnvHashMap<usize, usize>,
    next_value: usize,
}

impl MemoryGame {
    pub fn new() -> Self {
        MemoryGame {
            turn: 1,
            memory: FnvHashMap::default(),
            next_value: 0,
        }
    }

    pub fn insert_value(&mut self, value: usize) {
        if let Some(old_turn) = self.memory.insert(value, self.turn) {
            self.next_value = self.turn - old_turn;
        } else {
            self.next_value = 0
        }

        self.turn += 1;
    }

    pub fn generate_until(&mut self, value: usize) -> usize {
        while self.turn != value {
            self.insert_value(self.next_value);
        }

        self.next_value
    }
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day15, part1, naive)]
pub fn solve_part1_naive(input: &[usize]) -> usize {
    let mut mem_game = MemoryGame::new();
    input.iter().for_each(|x| mem_game.insert_value(*x));
    mem_game.generate_until(2020)
}

#[aoc(day15, part2, naive)]
pub fn solve_part2_naive(input: &[usize]) -> usize {
    let mut mem_game = MemoryGame::new();
    input.iter().for_each(|x| mem_game.insert_value(*x));
    mem_game.generate_until(30000000)
}
