// [X] parse input
//   [X] sensor positions
//   [X] sensor radius from manhattan distance to beacon
//   [X] grid bounds
// [X] manhattan distance
// [X] determine positions where the distriss beacon cannot be present
//   loop over all x positions in the specified y: determine if it is in the radius of one the sensors

use std::collections::HashSet;

use glam::IVec2;
use itertools::Itertools;

use crate::common::math::{IRect, IVec2Ext};

pub fn solve_part_one(input: String, row_to_check: i32) -> String {
    let sensors = parsing::input(&input);
    let beacons: HashSet<IVec2> = sensors.iter().map(|sensor| sensor.beacon).collect();
    let bounds = calculate_bounds(&sensors);
    (bounds.x1..=bounds.x2)
        .map(|x| IVec2::new(x, row_to_check))
        .filter(|&point| {
            !beacons.contains(&point)
                && sensors
                    .iter()
                    .any(|sensor| sensor.has_point_in_radius(point))
        })
        .count()
        .to_string()
}

pub fn solve_part_two(input: String, size: i32) -> String {
    let sensors = parsing::input(&input);
    let beacons: HashSet<IVec2> = sensors.iter().map(|sensor| sensor.beacon).collect();
    let distress_beacon_pos = (0..size)
        .cartesian_product(0..size)
        .map(|(x, y)| IVec2::new(x, y))
        .find(|&pos| {
            if beacons.contains(&pos) {
                return false;
            }

            sensors
                .iter()
                .all(|sensor| !sensor.has_point_in_radius(pos))
        })
        .unwrap();

    calculate_tuning_frequency(distress_beacon_pos).to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Sensor {
    pub center: IVec2,
    pub beacon: IVec2,
    radius: u32,
}

impl Sensor {
    pub fn new(center: IVec2, beacon: IVec2) -> Self {
        let radius = center.manhattan_distance(beacon);
        Self {
            center,
            beacon,
            radius,
        }
    }

    pub fn radius(self) -> u32 {
        self.radius
    }

    // (x - center_x)² + (y - center_y)² < radius²
    pub fn has_point_in_radius(self, beacon_pos: IVec2) -> bool {
        self.center.manhattan_distance(beacon_pos) <= self.radius()
    }
}

mod parsing {
    use nom::{
        bytes::complete::tag,
        character::complete::{self, line_ending},
        multi::separated_list1,
        sequence::preceded,
        IResult,
    };

    use super::*;

    pub(super) fn input(s: &str) -> Vec<Sensor> {
        let (_, sensors) = separated_list1(line_ending, sensor)(s).unwrap();

        sensors
    }

    fn sensor(s: &str) -> IResult<&str, Sensor> {
        let (s, _) = tag("Sensor at ")(s)?;
        let (s, sensor_center) = position(s)?;
        let (s, _) = tag(": closest beacon is at ")(s)?;
        let (s, beacon_pos) = position(s)?;
        let sensor = Sensor::new(sensor_center, beacon_pos);
        Ok((s, sensor))
    }

    fn position(s: &str) -> IResult<&str, IVec2> {
        let (s, x) = preceded(tag("x="), complete::i32)(s)?;
        let (s, _) = tag(", ")(s)?;
        let (s, y) = preceded(tag("y="), complete::i32)(s)?;
        Ok((s, IVec2::new(x, y)))
    }
}

fn calculate_bounds(sensors: &[Sensor]) -> IRect {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    for sensor in sensors {
        let min_radius_x = sensor.center.x - (sensor.radius() as i32);
        if min_radius_x < min_x {
            min_x = min_radius_x;
        }
        let max_radius_x = sensor.center.x + (sensor.radius() as i32);
        if max_radius_x > max_x {
            max_x = max_radius_x;
        }
        let min_radius_y = sensor.center.y - (sensor.radius() as i32);
        if min_radius_y < min_y {
            min_y = min_radius_y;
        }
        let max_radius_y = sensor.center.y + (sensor.radius() as i32);
        if max_radius_y > max_y {
            max_y = max_radius_y;
        }
    }

    IRect {
        x1: min_x,
        x2: max_x,
        y1: min_y,
        y2: max_y,
    }
}

fn calculate_tuning_frequency(pos: IVec2) -> i32 {
    pos.x * 4_000_000 + pos.y
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part_one() {
        let row_to_check = 10;
        let expected = "26";
        assert_eq!(solve_part_one(INPUT.to_string(), row_to_check), expected);
    }

    #[test]
    fn test_part_two() {
        let size = 20;
        let expected = "56000011";
        assert_eq!(solve_part_two(INPUT.to_string(), size), expected);
    }
}
