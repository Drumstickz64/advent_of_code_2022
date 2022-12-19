use itertools::Itertools;

use crate::common::iterator::is_all_unique;

pub fn solve_part_one(input: String) -> String {
    const MARKER_SIZE: usize = 4;
    find_marker_end_index(input, MARKER_SIZE).to_string()
}

pub fn solve_part_two(input: String) -> String {
    const MARKER_SIZE: usize = 14;
    find_marker_end_index(input, MARKER_SIZE).to_string()
}

fn find_marker_end_index(input: String, marker_size: usize) -> usize {
    let chars = input.chars().collect_vec();
    let start_index_of_marker = chars
        .windows(marker_size)
        .enumerate()
        .find(|(_, window)| is_all_unique(*window))
        .unwrap()
        .0;

    start_index_of_marker + marker_size
}
