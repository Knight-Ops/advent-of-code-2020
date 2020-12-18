use std::collections::VecDeque;

use eval::eval;
use factorial::Factorial;
use fnv::{FnvHashMap, FnvHashSet, FnvHasher};
use regex::{Captures, Regex};

lazy_static! {
    static ref MATH: Regex = Regex::new(r"^(\d+) (\*|\+) (\d+)").unwrap();
    static ref MATH_PLUS: Regex = Regex::new(r"(\d+) \+ (\d+)").unwrap();
    static ref MATH_MULT: Regex = Regex::new(r"(\d+) \* (\d+)").unwrap();
}

pub fn get_paren_slice(input: &str) -> &str {
    let mut layer = 0;
    let mut end = 0;
    for (idx, c) in input.chars().enumerate() {
        if c == '(' {
            layer += 1;
        } else if c == ')' {
            layer -= 1;

            if layer == 0 {
                end = idx;
                break;
            }
        }
    }

    &input[1..end]
}

pub fn eval_str(input: &str) -> usize {
    let mut eval_string = input.to_string();
    while MATH.is_match(&eval_string) {
        eval_string = MATH
            .replace(&eval_string, |caps: &Captures| {
                let lval = &caps[1].parse::<isize>().unwrap();
                let rval = &caps[3].parse::<isize>().unwrap();
                match &caps[2] {
                    "+" => format!("{}", lval + rval),
                    "*" => format!("{}", lval * rval),
                    _ => unreachable!(),
                }
            })
            .to_string();
    }
    eval_string.parse().unwrap()
}

pub fn eval_str_part2(input: &str) -> usize {
    let mut eval_string = input.to_string();
    while MATH_PLUS.is_match(&eval_string) {
        eval_string = MATH_PLUS
            .replace(&eval_string, |caps: &Captures| {
                let lval = &caps[1].parse::<isize>().unwrap();
                let rval = &caps[2].parse::<isize>().unwrap();
                format!("{}", lval + rval)
            })
            .to_string();
    }
    while MATH_MULT.is_match(&eval_string) {
        eval_string = MATH_MULT
            .replace(&eval_string, |caps: &Captures| {
                let lval = &caps[1].parse::<isize>().unwrap();
                let rval = &caps[2].parse::<isize>().unwrap();
                format!("{}", lval * rval)
            })
            .to_string();
    }
    eval_string.parse().unwrap()
}

pub fn resolve_parens(input: &str, eval_func: fn(&str) -> usize) -> String {
    if !input.contains('(') {
        return format!("{}", eval_func(input));
    } else {
        let mut resolved_string = input.to_string();
        let open_idx = resolved_string.find('(').unwrap();
        let paren_slice = get_paren_slice(&resolved_string[open_idx..]);
        let resolved_paren_string = resolve_parens(paren_slice, eval_func);
        resolved_string = resolved_string.replace(
            &format!("({})", paren_slice),
            &format!("{}", resolved_paren_string),
        );

        // println!("Resolved String : {}", resolved_string);

        resolved_string = resolve_parens(&resolved_string, eval_func);

        resolved_string
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> String {
    input.to_string()
}

#[aoc(day18, part1, naive)]
pub fn solve_part1_naive(input: &String) -> usize {
    input
        .lines()
        .map(|line| {
            // println!("{}", line);

            if line.contains('(') {
                let ret_str = resolve_parens(line, eval_str);
                eval_str(&ret_str)
            } else {
                eval_str(line)
            }
        })
        .sum()
}

/// Found eval 0.4.3, and then reweighted the Operations, theoretically you could probably do this
/// in rustc itself, but this was simpler, but turns out to be way slower even than my jenky String method.
#[aoc(day18, part1, eval)]
pub fn solve_part1_eval(input: &String) -> u64 {
    input
        .lines()
        .map(|line| eval(line).unwrap().as_u64().unwrap())
        .sum()
}

#[aoc(day18, part2, naive)]
pub fn solve_part2_naive(input: &String) -> usize {
    input
        .lines()
        .map(|line| {
            if line.contains('(') {
                let ret_str = resolve_parens(line, eval_str_part2);
                eval_str_part2(&ret_str)
            } else {
                eval_str_part2(line)
            }
        })
        .sum()
}
