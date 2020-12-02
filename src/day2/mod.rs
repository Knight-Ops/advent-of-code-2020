use regex::Regex;

lazy_static! {
    static ref PASSWORD_ENTRY_REGEX : Regex = Regex::new(r"(\d+)-(\d+) (.): (\w+)").unwrap();
}

#[derive(Debug, Clone)]
pub struct PasswordEntry {
    pub min: usize,
    pub max: usize,
    pub checked_char: char,
    pub password: String
}

impl PasswordEntry {
    pub fn from_str(input: &str) -> Self {
        let captures = PASSWORD_ENTRY_REGEX.captures(input).expect("Error parsing input with regex");

        PasswordEntry {
            min: captures.get(1).expect("Missing match for minimum value").as_str().parse().expect("Error parsing regex value for minimum value"),
            max: captures.get(2).expect("Missing match for maximum value").as_str().parse().expect("Error parsing regex value for maximum value"),
            checked_char: captures.get(3).expect("Missing match for check_char value").as_str().chars().next().expect("Error parsing regex value for checked_char"),
            password: captures.get(4).expect("Missing match for password value").as_str().to_string(),
        }

    }

    pub fn is_pass_within_limits(&self) -> bool {
        let chars = self.password.chars().filter(|&c| c == self.checked_char).count();

        self.min <= chars && chars <= self.max
    }

    pub fn do_pass_chars_match(&self) -> bool {
        (self.password.as_bytes()[self.min - 1] == self.checked_char as u8) ^ (self.password.as_bytes()[self.max - 1] == self.checked_char as u8)
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<PasswordEntry> {
    let vec : Vec<PasswordEntry> = input.lines().map(PasswordEntry::from_str).collect();

    vec
}

#[aoc(day2, part1)]
pub fn solve_part1(input:  &Vec<PasswordEntry>) -> usize {

    input.iter().filter(|pw| pw.is_pass_within_limits() == true).count()

}

#[aoc(day2, part2)]
pub fn solve_part2(input:  &Vec<PasswordEntry>) -> usize {

    input.iter().filter(|pw| pw.do_pass_chars_match() == true).count()

}