use fnv::FnvHashMap;
// use std::collections::HashSet;

/// Holds customs information for a single group of people
#[derive(Debug, Clone, Default)]
pub struct CustomsGroup {
    /// How many people are in each customs group
    size: usize,
    /// Questions answered yes to
    yes_questions: FnvHashMap<char, usize>,
}

impl CustomsGroup {
    pub fn from_str(input: &str) -> Self {
        let mut size = 0;
        let mut yes_questions = FnvHashMap::default();
        input.lines().for_each(|line| {
            size += 1;
            line.chars().for_each(|c| {
                *yes_questions.entry(c).or_insert(0) += 1;
            });
        });

        CustomsGroup {
            size,
            yes_questions,
        }
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<CustomsGroup> {
    let vec: Vec<CustomsGroup> = input
        // Split each `\n\n` which seperates each group
        .split("\n\n")
        // Parse these into `CustomsGroup`s
        .map(|data| CustomsGroup::from_str(data))
        // Collect it back into a vector
        .collect();

    vec
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[CustomsGroup]) -> usize {
    // Here we only care when *anyone* answered yes, so instead of caring about the contents of the map
    // We just use the size of the map, alternatively we could have used a Hashset as well, but then we
    // would need to construct 2 data structures for this problem
    input.iter().map(|cg| cg.yes_questions.len()).sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[CustomsGroup]) -> usize {
    input
        .iter()
        // Map our CustomsGroup into its `yes_questions` HashMap
        .map(|cg| {
            cg.yes_questions
                // Go through our values only, we don't care about keys
                .values()
                // Find values that show everyone in the group answered yes
                .filter(|val| **val == cg.size)
                // Count the number of those values
                .count()
        })
        // Then we sum the counts to find the total number of questions answered yes to (not uniquely) for all groups
        .sum()
}
