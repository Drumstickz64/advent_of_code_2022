use std::cmp::Ordering;

use itertools::Itertools;

pub fn solve_part_one(input: String) -> String {
    let packets = parsing::input(&input);

    packets
        .into_iter()
        .enumerate()
        .filter(|(_, (left, right))| left.cmp(right) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum::<usize>()
        .to_string()
}

pub fn solve_part_two(input: String) -> String {
    let packets = parsing::input(&input);

    let mut packets = packets
        .into_iter()
        .flat_map(|(left, right)| [left, right])
        .collect_vec();

    let divider_packets = [
        Packet::List(vec![Packet::List(vec![Packet::Number(6)])]),
        Packet::List(vec![Packet::List(vec![Packet::Number(2)])]),
    ];

    packets.extend(divider_packets.clone());

    packets.sort_unstable();

    packets
        .into_iter()
        .enumerate()
        .filter(|(_, packet)| divider_packets.contains(packet))
        .map(|(i, _)| i + 1)
        .product::<usize>()
        .to_string()
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Number(u32),
    List(Vec<Packet>),
}

impl Packet {
    fn compare_lists(list1: &[Packet], list2: &[Packet]) -> Ordering {
        for i in 0.. {
            match (list1.get(i), list2.get(i)) {
                (None, None) => return Ordering::Equal,
                (None, Some(_)) => return Ordering::Less,
                (Some(_), None) => return Ordering::Greater,
                (Some(left), Some(right)) => match left.cmp(right) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Equal => (),
                },
            }
        }

        unreachable!()
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Number(num1), Self::Number(num2)) => num1.cmp(num2),
            (Self::List(list1), Self::List(list2)) => Self::compare_lists(list1, list2),
            (Self::List(list), Self::Number(num)) => {
                Self::compare_lists(list, &[Packet::Number(*num)])
            }
            (Self::Number(num), Self::List(list)) => {
                Self::compare_lists(&[Packet::Number(*num)], list)
            }
        }
    }
}

mod parsing {
    use super::*;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{self, char, line_ending},
        multi::{separated_list0, separated_list1},
        sequence::{delimited, pair, separated_pair},
        IResult, Parser,
    };

    pub(super) fn input(input: &str) -> Vec<(Packet, Packet)> {
        let (_, packets) = separated_list1(
            pair(line_ending, line_ending),
            separated_pair(packet, line_ending, packet),
        )(input)
        .unwrap();

        packets
    }

    fn packet(input: &str) -> IResult<&str, Packet> {
        alt((
            delimited(
                tag("["),
                separated_list0(char(','), packet).map(Packet::List),
                tag("]"),
            ),
            complete::u32.map(Packet::Number),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part_one() {
        let expected = "13";
        assert_eq!(solve_part_one(INPUT.to_string()), expected);
    }

    #[test]
    fn test_part_two() {
        let expected = "140";
        assert_eq!(solve_part_two(INPUT.to_string()), expected);
    }
}
