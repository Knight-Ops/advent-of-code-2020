
use fnv::{FnvHashMap, FnvHashSet, FnvHasher};
use regex::Regex;

lazy_static! {
    static ref INSTRUCTIONS: Regex = Regex::new(r"([NSEWLFR])(\d+)").unwrap();
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize)
}

impl Instruction {
    pub fn from_str(input: &str) -> Self {
        let caps = INSTRUCTIONS.captures(input).expect("Error while capturing instruction from string");

        match &caps[1] {
            "N" => Instruction::North(caps[2].parse().expect("Error while parsing value associated with North instruction")),
            "S" => Instruction::South(caps[2].parse().expect("Error while parsing value associated with South instruction")),
            "E" => Instruction::East(caps[2].parse().expect("Error while parsing value associated with East instruction")),
            "W" => Instruction::West(caps[2].parse().expect("Error while parsing value associated with West instruction")),
            "L" => Instruction::Left(caps[2].parse().expect("Error while parsing value associated with Left instruction")),
            "R" => Instruction::Right(caps[2].parse().expect("Error while parsing value associated with Right instruction")),
            "F" => Instruction::Forward(caps[2].parse().expect("Error while parsing value associated with Forward instruction")),
            _ => unreachable!("Invalid instruction in input"),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North = 0,
    East,
    South,
    West
}

impl From<usize> for Direction {
    fn from(input: usize) -> Self {
        match input {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            _ => panic!("Invalid direction!")
        }
    }
}

impl Direction {
    fn wrapping_add(self, degrees:usize) -> Self {
        if degrees % 90 != 0 {
            panic!("We don't support turning in non 90 degree increments!");
        }

        let turns = degrees / 90;

        Direction::from(((self as u8) as usize + turns) % 4)
    }

    fn wrapping_sub(self, degrees:usize) -> Self {
        if degrees % 90 != 0 {
            panic!("We don't support turning in non 90 degree increments!");
        }

        let turns = degrees / 90;

        Direction::from((((self as u8) as usize).wrapping_sub(turns)) % 4)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Rotation {
    Left,
    Right
}

#[derive(Debug, Clone, Copy)]
pub struct Boat {
    boat_x_location: isize,
    boat_y_location: isize,
    waypoint_x_location: isize,
    waypoint_y_location: isize,
    facing: Direction,
}

impl Boat {
    pub fn new() -> Self {
        Boat {
            boat_x_location: 0,
            boat_y_location: 0,
            waypoint_x_location: 10,
            waypoint_y_location: 1,
            facing: Direction::East
        }
    }

    pub fn run(&mut self, instr: &Instruction) {
        match instr {
            Instruction::North(dist) => self.boat_y_location += *dist as isize,
            Instruction::South(dist) => self.boat_y_location -= *dist as isize,
            Instruction::East(dist) => self.boat_x_location += *dist as isize,
            Instruction::West(dist) => self.boat_x_location -= *dist as isize,
            Instruction::Left(degrees) => self.facing = self.facing.wrapping_sub(*degrees),
            Instruction::Right(degrees) =>self.facing = self.facing.wrapping_add(*degrees),
            Instruction::Forward(dist) => {
                match self.facing {
                    Direction::North => self.run(&Instruction::North(*dist)),
                    Direction::East =>self.run(&Instruction::East(*dist)),
                    Direction::South =>self.run(&Instruction::South(*dist)),
                    Direction::West =>self.run(&Instruction::West(*dist)),
                }
            }
        }
    }

    pub fn run_waypoint(&mut self, instr: &Instruction) {
        match instr {
            Instruction::North(dist) => self.waypoint_y_location += *dist as isize,
            Instruction::South(dist) => self.waypoint_y_location -= *dist as isize,
            Instruction::East(dist) => self.waypoint_x_location += *dist as isize,
            Instruction::West(dist) => self.waypoint_x_location -= *dist as isize,
            Instruction::Left(degrees) => self.rotate_waypoint(-(*degrees as isize)),
            Instruction::Right(degrees) =>self.rotate_waypoint(*degrees as isize),
            Instruction::Forward(dist) => {
                for _ in 0..*dist {
                    self.boat_x_location += self.waypoint_x_location;
                    self.boat_y_location += self.waypoint_y_location;
                }
            }
        }
    }

    fn rotate_waypoint(&mut self, degrees: isize) {
        if degrees % 90 != 0 {
            panic!("We don't support turning in non 90 degree increments!");
        }
        let mut rotation;
        let mut turns;

        if degrees < 0 {
            rotation = Rotation::Left;
            turns = degrees.abs() as usize / 90;

        } else if degrees > 0 {
            rotation = Rotation::Right;
            turns = degrees.abs() as usize / 90;
        } else {
            return
        }

        println!(" Turns : {}", turns);

        for _ in 0..turns {
            match rotation {
                Rotation::Right => {
                    let tmp = self.waypoint_y_location;

                    self.waypoint_x_location = self.waypoint_y_location;
                    self.waypoint_y_location = -tmp;

                },
                Rotation::Left => {
                    let tmp = self.waypoint_y_location;

                    self.waypoint_x_location = -self.waypoint_y_location;
                    self.waypoint_y_location = tmp;

                }
            }
        }


    }

    pub fn get_manhattan_distance(&self) -> usize {
        (self.boat_x_location.abs() + self.boat_y_location.abs()) as usize
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from_str).collect()
}

#[aoc(day12, part1, naive)]
pub fn solve_part1_naive(input: &[Instruction]) -> usize {
    let mut boat = Boat::new();
    for instr in input {
        boat.run(instr);
    }
    boat.get_manhattan_distance()
}

#[aoc(day12, part2, naive)]
pub fn solve_part2_naive(input: &[Instruction]) -> usize {
    let mut boat = Boat::new();
    for instr in input {
        boat.run_waypoint(instr);
    }
    // 95233 too high
    boat.get_manhattan_distance()
}
