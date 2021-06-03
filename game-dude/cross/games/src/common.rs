use core::ops::{Add, Sub};
use defmt::{self, Format};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Format)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    pub fn translate(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add for &Position {
    type Output = Position;

    fn add(self, other: Self) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub type Velocity = Position;

pub trait DiscreteSelection {
    fn next(self) -> Self;
    fn previous(self) -> Self;
}