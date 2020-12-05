use regex::Regex;

lazy_static! {
    static ref PASSPORT_ENTRIES: Regex = Regex::new(r"([a-z]{3}):([^ \n]*)").unwrap();
    static ref HEIGHT: Regex = Regex::new(r"(\d+)(in|cm)").unwrap();
    static ref HAIR_COLOR: Regex = Regex::new(r"#[0-9a-f]{6}$").unwrap();
}

/// Holds all passport data fields
#[derive(Debug, Clone, Default)]
pub struct PassportData {
    /// Birth Year
    byr: String,
    /// Issue Year
    iyr: String,
    /// Expiration Year
    eyr: String,
    /// Height
    hgt: String,
    /// Hair Color
    hcl: String,
    /// Eye Color
    ecl: String,
    /// Passport ID
    pid: String,
    /// Country ID
    cid: Option<String>,
}

impl PassportData {
    pub fn from_str(input: &str) -> Self {
        let mut init = PassportData::default();

        for cap in PASSPORT_ENTRIES.captures_iter(input) {
            match &cap[1] {
                "byr" => init.byr = cap[2].into(),
                "iyr" => init.iyr = cap[2].into(),
                "eyr" => init.eyr = cap[2].into(),
                "hgt" => init.hgt = cap[2].into(),
                "hcl" => init.hcl = cap[2].into(),
                "ecl" => init.ecl = cap[2].into(),
                "pid" => init.pid = cap[2].into(),
                "cid" => init.cid = Some(cap[2].into()),
                _ => {}
            }
        }

        init
    }

    /// Check that all fields, except `cid`, contain data of some sort by checking against the default
    /// value of `String`
    pub fn is_valid(&self) -> bool {
        self.byr != String::default()
            && self.iyr != String::default()
            && self.eyr != String::default()
            && self.hgt != String::default()
            && self.hcl != String::default()
            && self.ecl != String::default()
            && self.pid != String::default()
    }

    /// Check that all fields within the `PassportData` are valid given the constraints:
    /// byr (Birth Year) - four digits; at least 1920 and at most 2002.
    /// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    /// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    /// hgt (Height) - a number followed by either cm or in:
    ///     If cm, the number must be at least 150 and at most 193.
    ///     If in, the number must be at least 59 and at most 76.
    /// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    /// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    /// pid (Passport ID) - a nine-digit number, including leading zeroes.
    /// cid (Country ID) - ignored, missing or not.
    pub fn is_valid_constrained(&self) -> bool {
        is_valid_range(&self.byr, 1920, 2002)
            && is_valid_range(&self.iyr, 2010, 2020)
            && is_valid_range(&self.eyr, 2020, 2030)
            && self.is_height_valid()
            && self.is_hair_color_valid()
            && self.is_eye_color_valid()
            && self.is_passport_id_valid()
    }

    /// Check if `self.hgt` is valid by verifying it contains `in` or `cm` and check those
    /// values are within a set of bounds using `is_valid_range` generic helper
    fn is_height_valid(&self) -> bool {
        if let Some(caps) = HEIGHT.captures(&self.hgt) {
            match &caps[2] {
                "in" => is_valid_range(&caps[1], 59, 76),
                "cm" => is_valid_range(&caps[1], 150, 193),
                _ => false,
            }
        } else {
            false
        }
    }

    /// Check if `self.hcl` is valid by verifying it matches the format of a HTML hex color code
    /// "#[0-9a-f]{6}$" in Regex form
    fn is_hair_color_valid(&self) -> bool {
        if let Some(_) = HAIR_COLOR.captures(&self.hcl) {
            true
        } else {
            false
        }
    }

    /// Check if `self.ecl` is valid by verifying it is one of 7 valid variants
    fn is_eye_color_valid(&self) -> bool {
        match self.ecl.as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        }
    }

    /// Check if `self.pid` is valid by verifying the length is 9 and it parses into a valid number
    fn is_passport_id_valid(&self) -> bool {
        if self.pid.len() != 9 {
            return false;
        }

        if let Ok(_) = self.pid.parse::<usize>() {
            true
        } else {
            false
        }
    }
}

/// Generic helper that takes a &str, parses it to check it is a valid `usize` then checks it is not less than `min`
/// or greater than `max`
fn is_valid_range(value: &str, min: usize, max: usize) -> bool {
    if let Ok(val) = value.parse::<usize>() {
        if (min <= val) && (val <= max) {
            true
        } else {
            false
        }
    } else {
        false
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<PassportData> {
    let vec: Vec<PassportData> = input
        .split("\n\n")
        .map(|raw_passport| PassportData::from_str(raw_passport))
        .collect();

    vec
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[PassportData]) -> usize {
    input.iter().filter(|ppd| ppd.is_valid() == true).count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[PassportData]) -> usize {
    input
        .iter()
        .filter(|ppd| ppd.is_valid_constrained() == true)
        .count()
}
