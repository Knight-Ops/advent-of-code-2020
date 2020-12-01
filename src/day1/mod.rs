
static SEARCH_NUM : u32 = 2020;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    let mut vec : Vec<u32> = input.split("\n").into_iter().map(|num| num.parse::<u32>().expect("Error parsing &str into u32")).collect();

    vec.sort_unstable();

    vec
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    for needle in input {
        let find : Vec<u32> = input.iter().filter_map(|&haystack| {
            if needle + haystack == SEARCH_NUM {
                Some(needle * haystack)
            } else {
                None
            }
        }).collect();

        if find.len() != 0 {
            return find[0]
        } 
    }

    panic!("Unable to find any solutions!");
}

#[aoc(day1, part1, find)]
pub fn solve_part1_find(input: &[u32]) -> u32 {
    for needle in input {
        if let Some(answer) = input.iter().find_map(|&val| {
            if needle + val == SEARCH_NUM {
                Some(needle * val)
            } else {
                None
            }
        }) {
            return answer
        } 
    }

    panic!("Unable to find any solutions!");
}

#[aoc(day1, part1, par_iter)]
pub fn solve_part1_par_iter(input: &[u32]) -> u32 {
    use rayon::prelude::*;

    for needle in input {
        let find : Vec<u32> = input.par_iter().filter_map(|&haystack| {
            if needle + haystack == SEARCH_NUM {
                Some(needle * haystack)
            } else {
                None
            }
        }).collect();

        if find.len() != 0 {
            return find[0]
        } 
    }

    panic!("Unable to find any solutions!");
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {

    for needle in input {
        for haystack in input {
            let find : Vec<u32> = input.iter().filter_map(|&farmland| {
                if needle + haystack + farmland == SEARCH_NUM {
                    Some(needle * haystack * farmland)
                } else {
                    None
                }
            }).collect();
    
            if find.len() != 0 {
                return find[0]
            } 
        }
        
    }

    panic!("Unable to find any solutions!");
}

#[aoc(day1, part2, find)]
pub fn solve_part2_find(input: &[u32]) -> u32 {
    for needle in input {
        for haystack in input {
            if let Some(answer) = input.iter().find_map(|&farmland| {
                if needle + haystack + farmland == SEARCH_NUM {
                    Some(needle * haystack * farmland)
                } else {
                    None
                }
            }) {
                return answer
            } 
        }
    }

    panic!("Unable to find any solutions!");
}


// // Disabling this solution because the benchmarking is so slow, this is not the best way to solve it.
// // I am leaving it here just for the sake of already having implemented it.
// #[aoc(day1, part2, par_iter)]
// pub fn solve_part2_par_iter(input: &[u32]) -> u32 {
//     use rayon::prelude::*;

//     for needle in input {
//         for haystack in input {
//             let find : Vec<u32> = input.par_iter().filter_map(|&farmland| {
//                 if needle + haystack + farmland == SEARCH_NUM {
//                     Some(needle * haystack * farmland)
//                 } else {
//                     None
//                 }
//             }).collect();
    
//             if find.len() != 0 {
//                 return find[0]
//             } 
//         }
        
//     }

//     panic!("Unable to find any solutions!");
// }