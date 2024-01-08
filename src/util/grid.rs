use crate::util::point::*;
use std::ops::{Index, IndexMut};
use std::fmt::Display;

pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl Grid<u8> {
    pub fn parse(input: &[String]) -> Self {
        let width = input[0].len();
        let height = input.len();
        let mut data = Vec::with_capacity(width * height);
        input
            .iter()
            .for_each(|line| data.extend_from_slice(line.trim().as_bytes()));
        Grid {
            width,
            height,
            data,
        }
    }

    pub fn parse_str(input: &str) -> Self {
        Grid::parse(
            &input
                .lines()
                .map(|l| l.trim().to_owned())
                .collect::<Vec<_>>(),
        )
    }
}

impl<T: PartialEq> Grid<T> {
    #[inline]
    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0 && point.x < self.width as i32 && point.y >= 0 && point.y < self.height as i32
    }

    pub fn find(&self, needle: &T) -> Option<Point> {
        self.data
            .iter()
            .position(|x| x == needle)
            .map(|i| Point::new((i % self.width) as i32, (i / self.width) as i32))
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Point) -> &Self::Output {
        &self.data[index.y as usize * self.width + index.x as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.data[index.y as usize * self.width + index.x as usize]
    }
}

impl Display for Grid<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.data[y*self.width + x] as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
