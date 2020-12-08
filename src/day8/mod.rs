use fnv::{FnvHashMap, FnvHashSet};
use regex::Regex;

lazy_static! {
    static ref INSTRUCTIONS: Regex = Regex::new(r"(\w{3}) ([+-]\d+)").unwrap();
}

/// Holds the various reasons that the program execution exits
#[derive(Debug, Clone)]
pub enum ExitReason {
    /// If a loop is encountered `ExitReason::Loop` is created with the current value of the accumulator
    Loop(isize),
    /// If the instruction pointer does not point to an instruction `ExitReason::NoInstruction` is created
    NoInstruction,
}

/// Represents all possible instructions of the handheld device
#[derive(Debug, Clone)]
pub enum Instruction {
    /// No Operation
    NOP(isize),
    /// Accumulator
    ACC(isize),
    /// Jump
    JMP(isize),
}

impl Instruction {
    /// Parse an `Instruction` variant from a &str
    pub fn from_str(input: &str, val: &str) -> Self {
        match input {
            "nop" => Self::NOP(val.parse::<isize>().expect("Error parsing NOP value")),
            "acc" => Self::ACC(val.parse::<isize>().expect("Error parsing ACC value")),
            "jmp" => Self::JMP(val.parse::<isize>().expect("Error parsing JMP value")),
            _ => unreachable!(),
        }
    }

    /// Boolean check if Instruction is `Instruction:ACC(_)`, useful for filtering
    pub fn is_acc(&self) -> bool {
        match self {
            Self::ACC(_) => true,
            _ => false,
        }
    }
}

/// CPU Structure for the handheld
#[derive(Debug, Clone)]
pub struct CPU {
    /// Instruction pointer - Points at what instruction will be executed
    ip: usize,
    /// Accumulator - Register for accumulating values during execution
    acc: isize,
    /// Memory - Holds the address and `Instruction` for execution
    mem: FnvHashMap<usize, Instruction>,
    /// Patch list - Holds any patches that are made to the CPU memory for faster resets, holds the `self.ip` and
    /// the original Instruction
    patch_list: FnvHashMap<usize, Instruction>,
}

impl CPU {
    /// Create a new `CPU` based on the provided Instructions
    pub fn new(mem: &FnvHashMap<usize, Instruction>) -> Self {
        CPU {
            // Always start the CPU at address 0
            ip: 0,
            // Accumulator always starts at 0
            acc: 0,
            // This clone is a byproduct of restrictitons from cargo-aoc, we can't use a reference
            // since we can't pass lifetimes around, we can't parse in `CPU::new` because we can use
            // mutable references to part1 and part2. This should change when moving to any other runner
            mem: mem.clone(),
            // Empty Hashmap since we have no patches to start
            patch_list: FnvHashMap::default(),
        }
    }

    /// Reset the CPU to starting state
    pub fn reset(&mut self) {
        self.ip = 0;
        self.acc = 0;

        for (ip, instr) in &self.patch_list {
            *self.mem.get_mut(ip).unwrap() = instr.clone();
        }

        self.patch_list = FnvHashMap::default();
    }

    /// Execute a single instruction within the CPU
    fn execute(&mut self) -> Result<(), ExitReason> {
        // Try to get an instruction at `self.ip`
        if let Some(instr) = self.mem.get(&self.ip) {
            // Match the instruction for execution
            match instr {
                // NOP we do nothing except move on
                Instruction::NOP(_) => {
                    self.ip += 1;
                }
                // ACC we add to `self.acc` the value associated with the ACC instruction
                Instruction::ACC(val) => {
                    self.acc += val;
                    self.ip += 1;
                }
                // JMP we add to `self.ip` the value associated with the JMP instruction
                Instruction::JMP(val) => self.ip = self.ip.wrapping_add(*val as usize),
            }
            // Instruction was executed successfully
            Ok(())
        }
        // If an instruction doesn't exist, something happened, so we just exit
        else {
            Err(ExitReason::NoInstruction)
        }
    }

    /// Patch an opcode within the CPU instructions at the given `instr_addr`. This opcode is replaced with `new_instr`
    /// which is an `Instruction` enum variant closure, this is to allow the user to replace just the operation while
    /// not worrying about the value associated with the operation
    fn patch_opcode<T>(&mut self, instr_addr: usize, new_instr: T)
    where
        T: FnOnce(isize) -> Instruction,
    {
        // Add our patch to our patch_list so we can reset
        self.patch_list
            .insert(instr_addr, self.mem.get(&instr_addr).unwrap().clone());
        // Get a mutable reference to our current instruction in the CPU
        let current_instruction = self.mem.get_mut(&instr_addr).unwrap();
        // Unwrap the instruction to get its value, this feels clunky and there should be a better way to do this I think
        match current_instruction {
            // Destructure our current instruction to get its value, then give it to the new_instr closure
            Instruction::JMP(val) => *current_instruction = new_instr(*val),
            Instruction::ACC(val) => *current_instruction = new_instr(*val),
            Instruction::NOP(val) => *current_instruction = new_instr(*val),
        }
    }

    /// Specific swap for Part2, this is a less generalized version of `patch_opcode` just taking an `instr_addr` and
    /// swapping that instruction from a NOP to a JMP, or a JMP to a NOP changing nothing else.
    fn swap_jmp_nop(&mut self, instr_addr: usize) {
        let current_instruction = self.mem.get_mut(&instr_addr).unwrap();
        match current_instruction {
            Instruction::JMP(val) => *current_instruction = Instruction::NOP(*val),
            Instruction::NOP(val) => *current_instruction = Instruction::JMP(*val),
            _ => panic!("Invalid instruction to swap!"),
        }
    }

    /// Executes instructions within the CPU until a loop is detected, or no more instructions are found to execute
    pub fn run_until_loop(&mut self) -> Result<(), ExitReason> {
        // Build a hashset to track instructions we have executed. This is essentially code coverage tracking.
        let mut instruction_tracker = FnvHashSet::default();

        loop {
            // If we try to insert a `self.ip` that is already in the HashSet, then we are looping, so exit with the `self.acc`
            // value for Part 1
            if !instruction_tracker.insert(self.ip) {
                return Err(ExitReason::Loop(self.acc));
            }

            // Continue execution of the program
            self.execute()?
        }
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> FnvHashMap<usize, Instruction> {
    let mut mem = FnvHashMap::default();

    for (idx, cap) in INSTRUCTIONS.captures_iter(input).enumerate() {
        mem.insert(idx, Instruction::from_str(&cap[1], &cap[2]));
    }

    mem
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &FnvHashMap<usize, Instruction>) -> isize {
    let mut cpu = CPU::new(input);

    match cpu.run_until_loop() {
        Err(ExitReason::Loop(val)) => val,
        _ => unreachable!(),
    }
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &FnvHashMap<usize, Instruction>) -> isize {
    let solutions: Vec<isize> = input
        .iter()
        // Get instructions that are not Instruction::ACC(_) variants
        .filter(|(_, instr)| !instr.is_acc())
        // Build new CPUs for each attempt, we could maybe reset our current CPU instead
        .filter_map(|(ip, instr)| {
            // Build a new CPU
            let mut cpu = CPU::new(input);
            // Determine the specific instruction that we are looking at, and the correct patch for it
            let patch_op = match *instr {
                Instruction::JMP(_) => Instruction::NOP,
                Instruction::NOP(_) => Instruction::JMP,
                _ => unreachable!(),
            };
            // Patch the opcode in the CPU
            cpu.patch_opcode(*ip, patch_op);
            // Run the CPU
            match cpu.run_until_loop() {
                Ok(_) => None,
                Err(ExitReason::NoInstruction) => Some(cpu.acc),
                Err(ExitReason::Loop(_)) => None,
            }
        })
        // Collect our single solution into a vec, we could probably `.sum()` instead, but you wouldn't know if
        // for some reason this returned multiple values
        .collect();

    solutions[0]
}

#[aoc(day8, part2, reset)]
pub fn solve_part2_reset(input: &FnvHashMap<usize, Instruction>) -> isize {
    // Build a new CPU
    let mut cpu = CPU::new(input);
    let solutions: Vec<isize> = input
        .iter()
        // Get instructions that are not Instruction::ACC(_) variants
        .filter(|(_, instr)| !instr.is_acc())
        // Build new CPUs for each attempt, we could maybe reset our current CPU instead
        .filter_map(|(ip, instr)| {
            // reset the CPU
            cpu.reset();
            // Determine the specific instruction that we are looking at, and the correct patch for it
            let patch_op = match *instr {
                Instruction::JMP(_) => Instruction::NOP,
                Instruction::NOP(_) => Instruction::JMP,
                _ => unreachable!(),
            };
            // Patch the opcode in the CPU
            cpu.patch_opcode(*ip, patch_op);
            // Run the CPU
            match cpu.run_until_loop() {
                Ok(_) => None,
                Err(ExitReason::NoInstruction) => Some(cpu.acc),
                Err(ExitReason::Loop(_)) => None,
            }
        })
        // Collect our single solution into a vec, we could probably `.sum()` instead, but you wouldn't know if
        // for some reason this returned multiple values
        .collect();

    solutions[0]
}

#[aoc(day8, part2, swapcode)]
pub fn solve_part2_swap(input: &FnvHashMap<usize, Instruction>) -> isize {
    let solutions: Vec<isize> = input
        .iter()
        .filter(|(_, instr)| !instr.is_acc())
        .filter_map(|(ip, _)| {
            let mut cpu = CPU::new(input);
            cpu.swap_jmp_nop(*ip);
            match cpu.run_until_loop() {
                Ok(_) => None,
                Err(ExitReason::NoInstruction) => Some(cpu.acc),
                Err(ExitReason::Loop(_)) => None,
            }
        })
        .collect();

    solutions[0]
}
