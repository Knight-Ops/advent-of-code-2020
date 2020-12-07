use fnv::FnvHashMap;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::sync::Mutex;

lazy_static! {
    static ref LUGGAGE_RULES: Regex = Regex::new(r"(\d+) ([a-z ]+) (bags|bag)").unwrap();
    static ref CONTENTS_CACHE: Mutex<FnvHashMap<String, bool>> = Mutex::new(FnvHashMap::default());
    static ref SIZE_CACHE: Mutex<FnvHashMap<String, usize>> = Mutex::new(FnvHashMap::default());
}

/// Contains all of the Luggage Rules
#[derive(Debug, Clone, Default)]
pub struct LuggageRules {
    /// These rules are the bag color and a map of content bag color, and count
    rules: FnvHashMap<String, FnvHashMap<String, usize>>,
}

impl LuggageRules {
    pub fn from_str(input: &str) -> Self {
        // Build our default hashmap
        let mut rules = FnvHashMap::default();
        for line in input.lines() {
            // Split the parent bag color from the children bag color
            let split_input: Vec<&str> = line.split(" bags contain ").collect();
            // Make this a `String` because cargo-aoc doesn't let us use lifetimes
            let bag = split_input[0].to_string();

            let mut children = FnvHashMap::default();
            for cap in LUGGAGE_RULES.captures_iter(split_input[1]) {
                // Build our second hashmap which is the bag color and the number of that color
                // One hashmap is made for all children, assuming that a single color can occur once
                // in the contents list
                children.insert(
                    cap[2].to_string(),
                    cap[1]
                        .parse::<usize>()
                        .expect("Error parsing usize from input"),
                );
            }

            // Put our rule into our overarching ruleset
            rules.insert(bag, children);
        }

        LuggageRules { rules }
    }

    /// Search the specified `haystack` (one specific bag rule) for the specified `needle` (one specific bag color)
    /// returning `true` if `haystack` contains `needle`. This uses a recursive search of all of `needle`'s contents
    /// as well. This *should* benefit from cacheing if cargo-aoc would give mutable references.
    fn contains(&self, needle: &str, haystack: &str) -> bool {
        // Get the bag rule associated with haystack, this should never fail unless you search
        // for a bag color that doesn't exist in the ruleset
        if let Some(bag_contents) = self.rules.get(haystack) {
            // If our bag directly contains our needle, return true
            if let Some(_) = bag_contents.get(needle) {
                true
            } else {
                // If our bag doesn't directly contain our needle, we need to recursively search
                // our haystack's contents until we find it, or run out of contents to search.
                let mut contents_found = false;
                for key in bag_contents.keys() {
                    // If we find *any* bag that contains our needle, then we know everything above it contains
                    // the needle, so we can exit early to speed up our search. We should be able to cache this
                    // query to speed it up.
                    contents_found |= self.contains(needle, key);
                    if contents_found {
                        // Early return if we find *any* haystack that contains our needle
                        return contents_found;
                    }
                }
                // This is our long branch, if we don't find any needle in haystacks
                contents_found
            }
        } else {
            unreachable!("Hashmap should contain all possible searches")
        }
    }

    /// Search the specified `haystack` (one specific bag rule) for the specified `needle` (one specific bag color)
    /// returning `true` if `haystack` contains `needle`. This uses a recursive search of all of `needle`'s contents
    /// as well. This method uses caching of a global FnvHashMap behind a Mutex.
    fn contains_cached(&self, needle: &str, haystack: &str) -> bool {
        // Check our cache before we bother walking to find the answer ourselves
        if let Some(val) = CONTENTS_CACHE.lock().unwrap().get(haystack) {
            return *val;
        }

        // Get the bag rule associated with haystack, this should never fail unless you search
        // for a bag color that doesn't exist in the ruleset
        if let Some(bag_contents) = self.rules.get(haystack) {
            // If our bag directly contains our needle, return true
            if let Some(_) = bag_contents.get(needle) {
                true
            } else {
                // If our bag doesn't directly contain our needle, we need to recursively search
                // our haystack's contents until we find it, or run out of contents to search.
                let mut contents_found = false;
                for key in bag_contents.keys() {
                    // If we find *any* bag that contains our needle, then we know everything above it contains
                    // the needle, so we can exit early to speed up our search. We should be able to cache this
                    // query to speed it up.
                    contents_found |= self.contains_cached(needle, key);
                    if contents_found {
                        // Early return if we find *any* haystack that contains our needle
                        // Throw the solution into our cache before we exit
                        CONTENTS_CACHE
                            .lock()
                            .unwrap()
                            .insert(key.into(), contents_found);
                        return contents_found;
                    }
                }
                // This is our long branch, if we don't find any needle in haystacks
                // Throw the solution into our cache before we exit
                CONTENTS_CACHE
                    .lock()
                    .unwrap()
                    .insert(haystack.into(), contents_found);
                contents_found
            }
        } else {
            unreachable!("Hashmap should contain all possible searches")
        }
    }

    /// Calculates the number of bags **INCLUDING THE TOP BAG** that the provided needle contains (including itself)
    /// It does this by walking the list of contents of each bag, and recursively calculating the size of each bag.
    fn size(&self, needle: &str) -> usize {
        // Get our bag, this should only ever fial if you query for a bag that doesn't exist in the ruleset
        if let Some(bag_rule) = self.rules.get(needle) {
            // All bags count themselves (size of 1)
            let mut size = 1;
            // Iterate through all of the contents of the bag and add up all of their sizes recusively
            for (bag, num) in bag_rule {
                // Multiply the number of bags we have of each type, by their size, and that gives us the size of our
                // current bag
                size += num * self.size(bag);
            }
            // We will see a size of 1 for bags that have no contents (end nodes)
            size
        } else {
            unreachable!("Hashmap should contain all possible searches")
        }
    }

    /// Calculates the number of bags **INCLUDING THE TOP BAG** that the provided needle contains (including itself)
    /// It does this by walking the list of contents of each bag, and recursively calculating the size of each bag.
    /// This is cached in a global FnvHashMap behind a Mutex. Between the Mutex interactions and additional cache lookups
    /// this caching doesn't seem effective for smaller bag sizes, if you had more recursive bags, caching would quickly
    /// pull ahead in performance
    fn size_cached(&self, needle: &str) -> usize {
        // Check our cache before we bother walking to find the answer ourselves
        if let Some(size) = SIZE_CACHE.lock().unwrap().get(needle) {
            return *size;
        }

        // Get our bag, this should only ever fial if you query for a bag that doesn't exist in the ruleset
        if let Some(bag_rule) = self.rules.get(needle) {
            // All bags count themselves (size of 1)
            let mut size = 1;
            // Iterate through all of the contents of the bag and add up all of their sizes recusively
            for (bag, num) in bag_rule {
                // Multiply the number of bags we have of each type, by their size, and that gives us the size of our
                // current bag.
                size += num * self.size(bag);
            }
            // We will see a size of 1 for bags that have no contents (end nodes)
            // Whatever we find, we insert it into our cache for later cache hits
            SIZE_CACHE.lock().unwrap().insert(needle.into(), size);
            size
        } else {
            unreachable!("Hashmap should contain all possible searches")
        }
    }
}

impl std::fmt::Display for LuggageRules {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (bag, contents) in &self.rules {
            write!(f, "{} contains :\n", bag)?;
            for (bag, num) in contents {
                write!(f, "\t{} {}\n", num, bag)?;
            }
        }

        Ok(())
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> LuggageRules {
    LuggageRules::from_str(input)
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &LuggageRules) -> usize {
    let search_bag = "shiny gold";
    input
        .rules
        .iter()
        .filter(|(key, _)| input.contains(search_bag, key))
        .count()
}

#[aoc(day7, part1, contents_cached)]
pub fn solve_part1_cached(input: &LuggageRules) -> usize {
    let search_bag = "shiny gold";
    input
        .rules
        .iter()
        .filter(|(key, _)| input.contains_cached(search_bag, key))
        .count()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &LuggageRules) -> usize {
    let search_bag = "shiny gold";
    // We subtract 1 here because we don't want to count our outer-most bag
    input.size(search_bag) - 1
}

#[aoc(day7, part2, size_cached)]
pub fn solve_part2_cached(input: &LuggageRules) -> usize {
    let search_bag = "shiny gold";
    // We subtract 1 here because we don't want to count our outer-most bag
    input.size_cached(search_bag) - 1
}
