use std::ops::RangeInclusive;

pub fn solve_part_one(input: String) -> String {
    input
        .lines()
        .map(parse_pair)
        .filter(|(range1, range2)| {
            range_fully_contains(range1, range2) || range_fully_contains(range2, range1)
        })
        .count()
        .to_string()
}

pub fn solve_part_two(input: String) -> String {
    input
        .lines()
        .map(parse_pair)
        .filter(|(range1, range2)| ranges_overlap(range1, range2))
        .count()
        .to_string()
}

fn parse_pair(pair: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let mut ranges = pair.split(',').map(|range| {
        let mut ids = range.split('-').map(|id| id.parse::<i32>().unwrap());
        ids.next().unwrap()..=ids.next().unwrap()
    });
    (ranges.next().unwrap(), ranges.next().unwrap())
}

fn range_fully_contains(range1: &RangeInclusive<i32>, range2: &RangeInclusive<i32>) -> bool {
    range1.start() <= range2.start() && range1.end() >= range2.end()
}

fn ranges_overlap(range1: &RangeInclusive<i32>, range2: &RangeInclusive<i32>) -> bool {
    range1.contains(range2.start())
        || range1.contains(range2.end())
        || range2.contains(range1.start())
        || range2.contains(range1.end())
}
