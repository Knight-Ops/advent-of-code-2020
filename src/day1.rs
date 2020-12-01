
static SEARCH_NUM : u32 = 2020;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    let mut vec : Vec<u32> = input.split("\n").into_iter().map(|num| num.parse::<u32>().expect("Error parsing &str into u32")).collect();

    vec.sort_unstable();

    vec
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {

    let first = input;
    let second : Vec<&u32> = input.iter().rev().collect();

    for needle in first {
        let find : Vec<u32> = second.iter().filter_map(|&haystack| {
            if needle + haystack == 2020 {
                Some(needle * haystack)
            } else {
                None
            }
        }).collect();

        if find.len() != 0 {
            return find[0]
        } 
    }

    0
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {

    let first = input;
    let second = input;
    let third = input;

    for needle in first {
        for haystack in second {
            let find : Vec<u32> = third.iter().filter_map(|&farmland| {
                if needle + haystack + farmland == 2020 {
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

    0
}