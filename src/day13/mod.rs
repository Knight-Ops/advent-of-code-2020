
use fnv::{FnvHashMap, FnvHashSet, FnvHasher};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Schedule {
    leave_time : usize,
    bus_options: Vec<usize>,
}

impl Schedule {
    pub fn check_index(&self, value: usize, index: usize) -> bool {
        if index >= self.bus_options.len() {
            true
        } else if self.bus_options[index] == 0 {
            self.check_index(value, index + 1)
        } else {
            let check_value = (value / self.bus_options[index]) + 1;
            if check_value * self.bus_options[index] == value + index {
                self.check_index(value, index + 1)
            } else {
                false
            }
        }
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Schedule {
    let splits : Vec<&str> = input.split('\n').collect();

    let leave_time = splits[0].parse().unwrap();
    let bus_options = splits[1].split(',').map(|c| c.parse::<usize>().unwrap_or( 0)).collect();

    Schedule {
        leave_time,
        bus_options
    }
}

#[aoc(day13, part1, naive)]
pub fn solve_part1_naive(input: &Schedule) -> usize {
    let mut min_wait = usize::MAX;
    let mut answer = 0;

    for bus in input.bus_options.iter().filter(|x| **x != 0) {
        let wait_time = (bus * ((input.leave_time / bus) + 1)) - input.leave_time;

        if wait_time < min_wait {
            min_wait = wait_time;
            answer = wait_time * bus;
        }

    }

    answer
}

#[aoc(day13, part2, naive)]
pub fn solve_part2_naive(input: &Schedule) -> usize {
    let mut value = 0;
    // let mut value = 121977133361619;
    let mut max_bus = 0;
    let mut max_idx = 0;
    for (idx, bus) in input.bus_options.iter().enumerate() {
        if *bus > max_bus {
            max_bus = *bus;
            max_idx = idx;
        }
    }

    loop {
        value += input.bus_options[max_idx];


        if (value - max_idx) % input.bus_options[0] == 0 {
            let check = input.check_index(value - max_idx, 1);
            if check {
                println!("Found match!");
                return value
            }
        }

    }
}
