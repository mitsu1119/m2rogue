use std::ops::Range;
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    LEFT,
    UP,
    RIGHT,
    DOWN,
    DUMMY
}

impl Direction {
    pub fn go_x(&self, x: usize) -> usize {
        match self {
            Self::LEFT => x - 1,
            Self::RIGHT => x + 1,
            _ => x
        }
    }

    pub fn go_y(&self, y: usize) -> usize {
        match self {
            Self::UP => y - 1,
            Self::DOWN => y + 1,
            _ => y
        }
    }

    pub fn iter() -> Direction{
        Self::LEFT
    }
}

impl Iterator for Direction {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let (old, next) = match self {
            Self::LEFT => (Self::LEFT, Self::UP),
            Self::UP => (Self::UP, Self::RIGHT),
            Self::RIGHT => (Self::RIGHT, Self::DOWN),
            Self::DOWN => (Self::DOWN, Self::DUMMY),
            _ => (Self::DUMMY, Self::DUMMY)
        };

        if old == Self::DUMMY { None } else { *self = next; Some(old) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

impl Point {
    pub fn create(x: usize, y: usize) -> Self {
        return Self {x: x, y: y};
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Rect {
    pub s: Point,
    pub g: Point,
}

impl Rect {
    pub fn new() -> Self {
        return Default::default();
    }

    pub fn create(s: Point, g: Point) -> Self {
        return Self {s: s, g: g};
    }

    pub fn size(&self) -> Point {
        Point::create(self.g.x - self.s.x + 1, self.g.y - self.s.y + 1)
    }

    pub fn space(&self) -> usize {
        self.size().x * self.size().y
    }

    pub fn x_iter(&self) -> Range<usize> {
        self.s.x..(self.g.x + 1)
    }

    pub fn y_iter(&self) -> Range<usize> {
        self.s.y..(self.g.y + 1)
    }
}
