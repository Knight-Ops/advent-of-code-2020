use fnv::{FnvHashMap, FnvHashSet, FnvHasher};
use regex::Regex;

lazy_static! {
    static ref MASK: Regex = Regex::new(r"mask = ([01X]+)").unwrap();
    static ref MEM: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
}

#[derive(Debug, Clone)]
pub enum Op {
    Mask(String),
    Mem(usize, usize),
}

impl Op {
    pub fn from_str(input: &str) -> Self {
        if MASK.is_match(input) {
            let cap = MASK.captures(input).unwrap();
            Op::Mask(cap[1].to_string())
        } else if MEM.is_match(input) {
            let cap = MEM.captures(input).unwrap();
            Op::Mem(
                cap[1].parse::<usize>().unwrap(),
                cap[2].parse::<usize>().unwrap(),
            )
        } else {
            unreachable!("Invalid input given to Op::from_str")
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct BitmaskSystem {
    active_and_bitmask: usize,
    active_or_bitmask: usize,
    active_mask: String,
    resolved_memory_addresses: FnvHashSet<usize>,
    memory: FnvHashMap<usize, usize>,
}

impl BitmaskSystem {
    pub fn new() -> Self {
        BitmaskSystem {
            active_and_bitmask: usize::MAX,
            active_or_bitmask: 0,
            active_mask: String::default(),
            resolved_memory_addresses: FnvHashSet::default(),
            memory: FnvHashMap::default(),
        }
    }

    pub fn execute(&mut self, op: &Op) {
        match op {
            Op::Mask(mask) => {
                let (and_mask, or_mask) = self.parse(&mask);
                self.active_and_bitmask = and_mask;
                self.active_or_bitmask = or_mask;
            }
            Op::Mem(addr, val) => {
                self.memory.insert(*addr, self.apply_mask(*val));
            }
        }
    }

    pub fn execute_part2(&mut self, op: &Op) {
        match op {
            Op::Mask(mask) => {
                self.active_mask = mask.clone();
            }
            Op::Mem(addr, val) => {
                let masked_address = self.resolve_mask(*addr);
                self.resolved_memory_addresses.clear();
                self.resolve_floating_bits(masked_address);
                for resolved_addr in &self.resolved_memory_addresses {
                    self.memory.insert(*resolved_addr, *val);
                }
            }
        }
    }

    pub fn sum_memory(&self) -> usize {
        self.memory.iter().map(|(_, val)| val).sum()
    }

    fn parse(&self, mask: &str) -> (usize, usize) {
        let mut and_mask = usize::MAX;
        let mut or_mask = 0;

        mask.chars().rev().enumerate().for_each(|(idx, c)| match c {
            '0' => and_mask &= !(1 << idx),
            '1' => or_mask |= 1 << idx,
            'X' => {}
            _ => unreachable!(),
        });

        (and_mask, or_mask)
    }

    fn resolve_mask(&self, addr: usize) -> String {
        let mut local_string = String::with_capacity(self.active_mask.len());
        let mask_length = self.active_mask.len();
        format!("{:01$b}", addr, mask_length)
            .chars()
            .zip(self.active_mask.chars())
            .for_each(|(addr, mask)| match mask {
                '0' => local_string.push(addr),
                '1' => local_string.push('1'),
                'X' => local_string.push('X'),
                _ => unreachable!(),
            });

        local_string
    }

    fn resolve_floating_bits(&mut self, mask: String) {
        let mut local_str = mask;
        if let Some(idx) = local_str.find("X") {
            for x in "01".chars() {
                unsafe {
                    local_str.as_mut_str().as_bytes_mut()[idx] = x as u8;
                }
                self.resolve_floating_bits(local_str.clone());
            }
        } else {
            self.resolved_memory_addresses
                .insert(usize::from_str_radix(&local_str, 2).unwrap());
        }
    }

    fn apply_mask(&self, value: usize) -> usize {
        let mut masked_value = value;
        masked_value &= self.active_and_bitmask;
        masked_value |= self.active_or_bitmask;
        masked_value
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Op> {
    input.lines().map(Op::from_str).collect()
}

#[aoc(day14, part1, naive)]
pub fn solve_part1_naive(input: &[Op]) -> usize {
    let mut bitsys = BitmaskSystem::new();
    for i in input {
        bitsys.execute(i)
    }
    bitsys.sum_memory()
}

#[aoc(day14, part2, naive)]
pub fn solve_part2_naive(input: &[Op]) -> usize {
    let mut bitsys = BitmaskSystem::new();
    for i in input {
        bitsys.execute_part2(i)
    }
    bitsys.sum_memory()
}
