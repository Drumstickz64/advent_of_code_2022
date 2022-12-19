use std::{env, fs};

use advent_of_code_2022::{
    day1, day10, day11, day12, day13, day14, day15, day2, day3, day4, day5, day6, day7, day8, day9,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_number = args[1].parse::<u32>().unwrap();
    let puzzle_number = args[2].parse::<u32>().unwrap();
    let is_test = args.get(3) == Some(&String::from("test"));
    let input = read_input_of_day(day_number, is_test);
    let result: String = match (day_number, puzzle_number) {
        (1, 1) => day1::solve_part_one(input),
        (1, 2) => day1::solve_part_two(input),
        (2, 1) => day2::solve_part_one(input),
        (2, 2) => day2::solve_part_two(input),
        (3, 1) => day3::solve_part_one(input),
        (3, 2) => day3::solve_part_two(input),
        (4, 1) => day4::solve_part_one(input),
        (4, 2) => day4::solve_part_two(input),
        (5, 1) => day5::solve_part_one(input),
        (5, 2) => day5::solve_part_two(input),
        (6, 1) => day6::solve_part_one(input),
        (6, 2) => day6::solve_part_two(input),
        (7, 1) => day7::solve_part_one(input),
        (7, 2) => day7::solve_part_two(input),
        (8, 1) => day8::solve_part_one(input),
        (8, 2) => day8::solve_part_two(input),
        (9, 1) => day9::solve_part_one(input),
        (9, 2) => day9::solve_part_two(input),
        (10, 1) => day10::solve_part_one(input),
        (10, 2) => day10::solve_part_two(input),
        (11, 1) => day11::solve_part_one(input),
        (11, 2) => day11::solve_part_two(input),
        (12, 1) => day12::solve_part_one(input),
        (12, 2) => day12::solve_part_two(input),
        (13, 1) => day13::solve_part_one(input),
        (13, 2) => day13::solve_part_two(input),
        (14, 1) => day14::solve_part_one(input),
        (14, 2) => day14::solve_part_two(input),
        (15, 1) => day15::solve_part_one(input, 2_000_000),
        (15, 2) => day15::solve_part_two(input, 4_000_000),
        _ => panic!("Day {day_number} or puzzle {puzzle_number} not found"),
    };
    println!("Result:\n{result}");
}

fn read_input_of_day(day_number: u32, is_test: bool) -> String {
    let filename = if is_test {
        "test_input.txt"
    } else {
        "input.txt"
    };
    let input_path = format!("./src/day{day_number}/{filename}");
    fs::read_to_string(input_path).unwrap()
}
