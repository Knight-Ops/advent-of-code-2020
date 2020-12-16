use fnv::{FnvHashMap, FnvHashSet};
use regex::Regex;
use std::ops::RangeInclusive;

lazy_static! {
    static ref FIELDS: Regex = Regex::new(r"([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ticket {
    values: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct TrainTickets {
    fields: FnvHashMap<String, FnvHashSet<RangeInclusive<usize>>>,
    my_ticket: Ticket,
    other_tickets: FnvHashSet<Ticket>,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> TrainTickets {
    let groups: Vec<&str> = input.split("\n\n").collect();

    let mut fields = FnvHashMap::default();
    for field in FIELDS.captures_iter(groups[0]) {
        let mut ranges: FnvHashSet<RangeInclusive<usize>> = FnvHashSet::default();

        let first_range = RangeInclusive::new(
            field[2].parse::<usize>().unwrap(),
            field[3].parse::<usize>().unwrap(),
        );
        ranges.insert(first_range);

        let second_range = RangeInclusive::new(
            field[4].parse::<usize>().unwrap(),
            field[5].parse::<usize>().unwrap(),
        );
        ranges.insert(second_range);

        fields.insert(String::from(&field[1]), ranges);
    }

    let my_ticket: Ticket = match groups[1].splitn(2, "\n").skip(1).next() {
        Some(str_ticket) => {
            let mut ticket_vec = Vec::new();
            str_ticket
                .split(',')
                .for_each(|x| ticket_vec.push(x.parse::<usize>().unwrap()));
            Ticket { values: ticket_vec }
        }
        None => unreachable!(),
    };

    let mut other_tickets = FnvHashSet::default();
    groups[2].splitn(2, "\n").skip(1).for_each(|str_ticket| {
        str_ticket.lines().for_each(|line| {
            let mut ticket_vec = Vec::new();
            line.split(',')
                .for_each(|x| ticket_vec.push(x.parse::<usize>().unwrap()));
            other_tickets.insert(Ticket { values: ticket_vec });
        });
    });

    TrainTickets {
        fields,
        my_ticket,
        other_tickets,
    }
}

#[aoc(day16, part1, naive)]
pub fn solve_part1_naive(input: &TrainTickets) -> usize {
    let mut error_rate = 0;

    for ticket in &input.other_tickets {
        for value in &ticket.values {
            let mut error = false;
            for field in input.fields.values() {
                for range in field {
                    error |= range.contains(value);
                }
            }
            if !error {
                error_rate += value;
            }
        }
    }
    error_rate
}

#[aoc(day16, part2, naive)]
pub fn solve_part2_naive(input: &TrainTickets) -> usize {
    let mut mutable_train_tickets = input.clone();

    for ticket in &input.other_tickets {
        for value in &ticket.values {
            let mut error = false;
            for field in input.fields.values() {
                for range in field {
                    error |= range.contains(value);
                }
            }
            if !error {
                mutable_train_tickets.other_tickets.remove(ticket);
            }
        }
    }

    let mut fieldIndexes: FnvHashMap<String, Vec<usize>> = FnvHashMap::default();
    for (fieldname, fieldRanges) in &mutable_train_tickets.fields {
        'index_search: for x in 0..mutable_train_tickets.my_ticket.values.len() {
            for ot in &mutable_train_tickets.other_tickets {
                let mut idx_value_in_ranges = false;
                let idx_value = ot.values[x];
                for range in fieldRanges {
                    idx_value_in_ranges |= range.contains(&idx_value);
                }

                if !idx_value_in_ranges {
                    continue 'index_search;
                }
            }

            let map_idx = fieldIndexes.entry(fieldname.clone()).or_insert(vec![]);
            map_idx.push(x);
        }
    }

    let concrete = remove_duplicate_indexes(&mut fieldIndexes);

    concrete
        .iter()
        .filter(|(key, _)| key.starts_with("departure"))
        .map(|(_, val)| mutable_train_tickets.my_ticket.values[*val])
        .product()
}

fn remove_duplicate_indexes(
    input: &mut FnvHashMap<String, Vec<usize>>,
) -> FnvHashMap<String, usize> {
    let mut concrete_results = FnvHashMap::default();

    loop {
        for (field, idxs) in input.iter() {
            let mut tmp_idxs = idxs.clone();
            for (_, val) in concrete_results.iter() {
                for (tmp_idx, tmp_val) in tmp_idxs.clone().iter().enumerate() {
                    if tmp_val == val {
                        tmp_idxs.remove(tmp_idx);
                    }
                }
            }
            if tmp_idxs.len() == 1 {
                concrete_results.insert(field.clone(), tmp_idxs[0]);
            }
        }

        if concrete_results.len() == input.len() {
            break;
        }
    }

    concrete_results
}
