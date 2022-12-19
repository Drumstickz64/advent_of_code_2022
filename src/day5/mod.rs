use std::collections::VecDeque;

use itertools::Itertools;

use crate::common::string::ascii_char_at;

pub fn solve_part_one(input: String) -> String {
    let (mut crates, instructions) = parse_input(&input);
    for instruction in instructions {
        for _ in 0..instruction.quantity {
            let to_add = crates[instruction.from].pop().unwrap();
            crates[instruction.to].push(to_add);
        }
    }

    get_final_output(crates)
}

pub fn solve_part_two(input: String) -> String {
    let (mut crates, instructions) = parse_input(&input);
    for instruction in instructions {
        let mut buf = VecDeque::new();
        for _ in 0..instruction.quantity {
            let to_add = crates[instruction.from].pop().unwrap();
            buf.push_front(to_add);
        }
        crates[instruction.to].extend(buf);
    }

    get_final_output(crates)
}

type Crate = char;

#[derive(Debug)]
struct Instruction {
    pub quantity: usize,
    pub from: usize,
    pub to: usize,
}

fn parse_input(input: &str) -> (Vec<Vec<Crate>>, Vec<Instruction>) {
    let mut split = input.split("\n\n");
    let crates_area = split.next().unwrap();
    let instructions_area = split.next().unwrap();
    (
        parse_crates_area(crates_area),
        parse_instructions_area(instructions_area),
    )
}

fn parse_crates_area(crates_area: &str) -> Vec<Vec<Crate>> {
    let mut lines = crates_area.lines().collect_vec();
    lines.pop(); // remove the unnecessary numbers section at the bottom
    let strings_len = lines[0].len();
    (1..strings_len)
        .step_by(4)
        .map(|i| {
            lines
                .iter()
                .rev()
                .map(|stack| ascii_char_at(stack, i))
                .take_while(|&ch| ch != ' ')
                .collect_vec()
        })
        .collect_vec()
}

fn parse_instructions_area(instructions_area: &str) -> Vec<Instruction> {
    instructions_area
        .lines()
        .map(|line| {
            let mut sections = line.split_ascii_whitespace();
            let quantity = sections.nth(1).unwrap();
            let from = sections.nth(1).unwrap();
            let to = sections.nth(1).unwrap();
            Instruction {
                quantity: quantity.parse().unwrap(),
                from: from.parse::<usize>().unwrap() - 1,
                to: to.parse::<usize>().unwrap() - 1,
            }
        })
        .collect()
}

fn get_final_output(crates: Vec<Vec<Crate>>) -> String {
    crates
        .into_iter()
        .map(|crate_| *crate_.last().unwrap())
        .collect()
}
