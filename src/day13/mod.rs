use fnv::{FnvHashMap, FnvHashSet, FnvHasher};
use rayon::prelude::*;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Schedule {
    leave_time: usize,
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

    fn compute_n(&self) -> usize {
        self.bus_options.iter().filter(|x| **x != 0).product()
    }

    fn compute_yi(&self, n: usize) -> Vec<usize> {
        self.bus_options
            .iter()
            .map(|val| if *val == 0 { 0 } else { n / val })
            .collect()
    }

    fn compute_zi(&self, yi: &Vec<usize>) -> Vec<usize> {
        self.bus_options
            .iter()
            .zip(yi)
            .map(|(ni, yi)| {
                if *ni == 0 {
                    0
                } else {
                    let mut zi = 1;

                    while (yi * zi) % ni != 1 {
                        zi += 1;
                    }

                    zi
                }
            })
            .collect()
    }

    fn compute_x(&self, yi: &Vec<usize>, zi: &Vec<usize>) -> usize {
        yi.iter()
            .zip(zi)
            .enumerate()
            .filter(|(_, (yx, zx))| **yx != 0)
            .map(|(idx, (yx, zx))| idx * yx * zx)
            .sum()
    }

    fn crt(&self) -> usize {
        let n = self.compute_n();
        let yi = self.compute_yi(n);
        let zi = self.compute_zi(&yi);
        self.compute_x(&yi, &zi) % n
    }

    pub fn get_first_unique_crt_solution(&self) -> usize {
        let n = self.compute_n();

        n - self.crt()
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Schedule {
    let splits: Vec<&str> = input.split('\n').collect();

    let leave_time = splits[0].parse().unwrap();
    let bus_options = splits[1]
        .split(',')
        .map(|c| c.parse::<usize>().unwrap_or(0))
        .collect();

    Schedule {
        leave_time,
        bus_options,
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

// #[aoc(day13, part2, naive)]
// pub fn solve_part2_naive(input: &Schedule) -> usize {
//     let mut value = 0;
//     // let mut value = 121977133361619;
//     let mut max_bus = 0;
//     let mut max_idx = 0;
//     for (idx, bus) in input.bus_options.iter().enumerate() {
//         if *bus > max_bus {
//             max_bus = *bus;
//             max_idx = idx;
//         }
//     }

//     loop {
//         value += input.bus_options[max_idx];

//         if (value - max_idx) % input.bus_options[0] == 0 {
//             let check = input.check_index(value - max_idx, 1);
//             if check {
//                 println!("Found match!");
//                 return value
//             }
//         }

//     }
// }

// #[aoc(day13, part2, parallel)]
// pub fn solve_part2_parallel(input: &Schedule) -> usize {
//     let mut max_bus = 0;
//     let mut max_idx = 0;
//     for (idx, bus) in input.bus_options.iter().enumerate() {
//         if *bus > max_bus {
//             max_bus = *bus;
//             max_idx = idx;
//         }
//     }

//     let answer = std::sync::atomic::AtomicUsize::new(0);

//     (0..24_usize).into_par_iter().for_each(|core| {
//         let mut core_iter = 0;
//         let mut value = (672600000000000 / input.bus_options[max_idx]) * input.bus_options[max_idx];
//         loop {
//             value += input.bus_options[max_idx] * (core + 1);

//             if value == 672754131923874 + max_idx {
//                 println!("Found value on core : {} | {}", core, (value - max_idx) % input.bus_options[0] == 0);
//             }

//             if core_iter % 0x8000000 == 0 {
//                 if answer.load(std::sync::atomic::Ordering::Relaxed) != 0 {
//                     return;
//                 }
//             }

//             if (value - max_idx) % input.bus_options[0] == 0 {
//                 let check = input.check_index(value - max_idx, 1);
//                 if check {
//                     answer.store(value, std::sync::atomic::Ordering::SeqCst);
//                 }
//             }

//             if core == 0 {
//                 if core_iter % 0x8000000 == 0 {
//                     println!("Current Core 0 Value : {}", value);
//                 }
//             }
//             core_iter += 1;
//         }
//     });
//     answer.into_inner()
// }

#[aoc(day13, part2, crt)]
pub fn solve_part2_crt(input: &Schedule) -> usize {
    input.get_first_unique_crt_solution()
}
