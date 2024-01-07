use std::ops::{Add, AddAssign, Sub, SubAssign, Mul};

pub const ORIGIN: Point = Point::new(0, 0);
pub const UP: Point = Point::new(0, 1);
pub const DOWN: Point = Point::new(0, -1);
pub const LEFT: Point = Point::new(-1, 0);
pub const RIGHT: Point = Point::new(1, 0);
pub const ORTHOGONALS: [Point; 4] = [UP, DOWN, LEFT, RIGHT];
// Left to right, top to bottom
pub const DIAGONALS: [Point; 8] = [
    Point::new(-1, 1),
    UP,
    Point::new(1, 1),
    RIGHT,
    Point::new(1, -1),
    DOWN,
    Point::new(-1, -1),
    LEFT,
];

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    #[inline]
    #[must_use]
    pub fn manhattan_distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    #[inline]
    #[must_use]
    pub fn counter_clockwise(self) -> Self {
        Point::new(-self.y, self.x)
    }

    #[inline]
    #[must_use]
    pub fn clockwise(self) -> Self {
        Point::new(self.y, -self.x)
    }
}

impl From<u8> for Point {
    #[inline]
    #[must_use]
    fn from(value: u8) -> Self {
        match value {
            b'>' | b'R' => RIGHT,
            b'<' | b'L' => LEFT,
            b'^' | b'U' => UP,
            b'v' | b'D' => DOWN,
            _ => panic!("Unknown direction: {}", value),
        }
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn mul(self, rhs: i32) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}
