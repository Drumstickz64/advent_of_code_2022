use std::collections::HashSet;
use std::fmt::Write;

use glam::IVec2;

const SAND_ORIGIN: IVec2 = IVec2::new(500, 0);
const MOVE_DIRECTIONS: [IVec2; 3] = [IVec2::Y, IVec2::new(-1, 1), IVec2::ONE];

pub fn solve_part_one(input: String) -> String {
    let mut tiles = parsing::input(&input);
    for i in 0.. {
        let mut sand_pos = SAND_ORIGIN;
        let mut moving = true;
        while moving {
            moving = false;
            for move_dir in MOVE_DIRECTIONS {
                let next_pos = sand_pos + move_dir;
                if !tiles.contains(&next_pos) {
                    sand_pos = next_pos;
                    moving = true;
                    break;
                }
            }
            if !tiles
                .iter()
                .any(|tile_pos| tile_pos.x == sand_pos.x && tile_pos.y > sand_pos.y)
            {
                return i.to_string();
            }
        }
        tiles.insert(sand_pos);
    }

    unreachable!()
}

pub fn solve_part_two(input: String) -> String {
    let mut tiles = parsing::input(&input);
    let mut buf = String::new();
    let floor = Floor::from_entries(&tiles);
    for i in 0.. {
        let mut sand_pos = SAND_ORIGIN;
        let mut moving = true;
        while moving {
            moving = false;
            for move_dir in MOVE_DIRECTIONS {
                let next_pos = sand_pos + move_dir;
                if !tiles.contains(&next_pos) && !floor.is_floor(next_pos) {
                    sand_pos = next_pos;
                    moving = true;
                    break;
                }
            }
        }

        writeln!(&mut buf, "({},{})", sand_pos.x * 100, sand_pos.y * -100).unwrap();

        if sand_pos == SAND_ORIGIN {
            std::fs::write("out.txt", buf).unwrap();
            return (i + 1).to_string();
        }
        tiles.insert(sand_pos);
    }

    unreachable!()
}

mod parsing {
    use std::collections::HashSet;

    use glam::IVec2;
    use nom::{
        bytes::complete::tag,
        character::complete::{self, line_ending},
        combinator::map,
        multi::separated_list1,
        sequence::separated_pair,
        IResult,
    };

    pub(super) fn input(input: &str) -> HashSet<IVec2> {
        let mut rocks = HashSet::new();
        let (_, vertices) = vertices(input).unwrap();
        for line in vertices {
            for win in line.windows(2) {
                let [from, to] = *win else { unreachable!() };
                let line = to - from;
                for step in 0..=line.abs().max_element() {
                    rocks.insert(from + line.signum() * step);
                }
            }
        }
        rocks
    }

    fn vertices(s: &str) -> IResult<&str, Vec<Vec<IVec2>>> {
        separated_list1(line_ending, line)(s)
    }

    fn line(s: &str) -> IResult<&str, Vec<IVec2>> {
        separated_list1(tag(" -> "), point)(s)
    }

    fn point(s: &str) -> IResult<&str, IVec2> {
        map(
            separated_pair(complete::i32, complete::char(','), complete::i32),
            |(x, y)| IVec2::new(x, y),
        )(s)
    }
}

struct Floor(i32);

impl Floor {
    pub fn from_entries(entries: &HashSet<IVec2>) -> Self {
        let y = entries.iter().map(|pos| pos.y).max().unwrap() + 2;
        Self(y)
    }

    pub fn is_floor(&self, pos: IVec2) -> bool {
        pos.y >= self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part_one() {
        let expected = "24";
        assert_eq!(solve_part_one(INPUT.to_string()), expected);
    }

    #[test]
    fn test_part_two() {
        let expected = "93";
        assert_eq!(solve_part_two(INPUT.to_string()), expected);
    }
}
