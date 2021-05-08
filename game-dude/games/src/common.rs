use crate::graphics::Point;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position {
            x,
            y,
        }
    }

    pub fn translate(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}

impl From<&Position> for Point {
    fn from(p: &Position) -> Self {
        Point::new(p.x, p.y)
    }
}

pub type Velocity = Position;

pub trait MovingObject {
    fn update_position(&mut self);
}
