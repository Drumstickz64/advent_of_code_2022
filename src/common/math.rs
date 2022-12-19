use glam::{IVec2, Vec2};

pub trait Vec2Ext: Copy {
    fn to_direction(self) -> Self;
}

pub fn to_direction(num: f32) -> f32 {
    if num == 0.0 || num.is_nan() {
        num
    } else if num > 0.0 {
        1.0
    } else {
        -1.0
    }
}

impl Vec2Ext for Vec2 {
    fn to_direction(self) -> Self {
        Self::new(to_direction(self.x), to_direction(self.y))
    }
}

pub trait IVec2Ext: Copy {
    fn manhattan_distance(&self, other: Self) -> u32;
}

impl IVec2Ext for IVec2 {
    // |p1.x - p2.x| + |p1.y - p2.y|
    fn manhattan_distance(&self, other: Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IRect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}
