use std::collections::HashSet;

use crate::common::string::alphabet_score;

pub fn solve_part_one(input: String) -> String {
    input
        .lines()
        .map(|rucksack| {
            let middle_index = rucksack.len() / 2;
            let (first_compartment, second_compartment) = rucksack.split_at(middle_index);
            let first_compartment_items: HashSet<Item> = first_compartment.chars().collect();
            let second_compartment_items: HashSet<Item> = second_compartment.chars().collect();
            *first_compartment_items
                .intersection(&second_compartment_items)
                .next()
                .unwrap()
        })
        .map(alphabet_score)
        .sum::<u32>()
        .to_string()
}

pub fn solve_part_two(input: String) -> String {
    let lines = input.lines().collect::<Vec<_>>();
    lines
        .chunks_exact(3)
        .map(|group| {
            group
                .iter()
                .map(|rucksack| rucksack.chars().collect::<HashSet<_>>())
                .reduce(|acc, val| acc.intersection(&val).copied().collect::<HashSet<_>>())
                .unwrap()
                .into_iter()
                .next()
                .unwrap()
        })
        .map(alphabet_score)
        .sum::<u32>()
        .to_string()
}

type Item = char;
