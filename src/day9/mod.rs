use std::collections::HashSet;

use glam::{IVec2, Vec2};
use itertools::Itertools;

use crate::common::math::Vec2Ext;

pub fn solve_part_one(input: String) -> String {
    let moves = parse_input(&input);
    let mut head_position = Vec2::default();
    let mut tail_position = Vec2::default();
    let mut tail_positions: HashSet<IVec2> = HashSet::from([IVec2::ZERO]);
    for mov in moves {
        // move head
        head_position += mov;
        // check distance from head to tail
        let distance = head_position.distance(tail_position);
        // move tail
        if distance >= 2.0 {
            let direction = (head_position - tail_position)
                .normalize_or_zero()
                .to_direction();
            tail_position += direction;
            tail_positions.insert(tail_position.as_ivec2());
        }
    }
    tail_positions.len().to_string()
}

pub fn solve_part_two(input: String) -> String {
    const NUM_KNOTS: usize = 10;
    let moves = parse_input(&input);
    let mut knot_positions = [Vec2::ZERO; NUM_KNOTS];
    let mut tail_positions: HashSet<IVec2> = HashSet::from([IVec2::ZERO]);
    for mov in moves {
        // move head
        knot_positions[0] += mov;
        // check distance from first to second
        for i in 0..NUM_KNOTS - 1 {
            let distance = knot_positions[i].distance(knot_positions[i + 1]);
            // move tail
            if distance >= 2.0 {
                let direction = (knot_positions[i] - knot_positions[i + 1])
                    .normalize_or_zero()
                    .to_direction();
                knot_positions[i + 1] += direction;
            }
        }
        tail_positions.insert(knot_positions.last().unwrap().as_ivec2());
    }
    tail_positions.len().to_string()
}

fn parse_input(input: &str) -> Vec<Vec2> {
    input
        .lines()
        .flat_map(|line| {
            let (direction, count) = line.split_ascii_whitespace().collect_tuple().unwrap();
            let direction = direction_from_letter(direction);
            let count: usize = count.parse().unwrap();
            std::iter::repeat(direction).take(count)
        })
        .collect_vec()
}

fn direction_from_letter(letter: &str) -> Vec2 {
    match letter {
        "L" => Vec2::NEG_X,
        "U" => Vec2::Y,
        "R" => Vec2::X,
        "D" => Vec2::NEG_Y,
        _ => panic!("Invalid direction letter: {letter}"),
    }
}
